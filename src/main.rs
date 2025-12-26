use anyhow::{Context, Result};
use clap::Parser;
use rayon::prelude::*;
use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::fs;
use std::path::PathBuf;
use url::Url;

#[derive(Parser, Debug)]
#[command(name = "rss-miner")]
#[command(about = "Finds RSS feeds from URLs and generates an OPML file", long_about = None)]
struct Args {
    /// Input file containing URLs (one per line)
    #[arg(short, long, value_name = "FILE")]
    input: PathBuf,

    /// Output OPML file path
    #[arg(short, long, value_name = "FILE", default_value = "feeds.opml")]
    output: PathBuf,
}

#[derive(Debug, Clone)]
struct RssFeed {
    title: String,
    url: String,
    html_url: String,
    feed_type: FeedType,
}

#[derive(Debug, Clone)]
enum FeedType {
    Rss,
    Atom,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Read URLs from input file
    let urls = read_urls_from_file(&args.input)?;
    println!("Found {} URLs to process", urls.len());

    // Create a shared HTTP client for all operations
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    // Find RSS feeds in parallel using Rayon
    let feeds: Vec<RssFeed> = urls
        .par_iter()
        .filter_map(|url| {
            println!("Processing: {}", url);
            match find_rss_feeds(url, &client) {
                Ok(feeds) => {
                    if !feeds.is_empty() {
                        println!("  Found {} feed(s) for {}", feeds.len(), url);
                        Some(feeds)
                    } else {
                        println!("  No feeds found for {}", url);
                        None
                    }
                }
                Err(e) => {
                    eprintln!("  Error processing {}: {}", url, e);
                    None
                }
            }
        })
        .flatten()
        .collect();

    println!("\nTotal feeds found: {}", feeds.len());

    if feeds.is_empty() {
        println!("No RSS feeds found. OPML file will not be created.");
        return Ok(());
    }

    // Create OPML file
    create_opml_file(&feeds, &args.output)?;
    println!("OPML file created: {}", args.output.display());

    Ok(())
}

fn read_urls_from_file(path: &std::path::Path) -> Result<Vec<String>> {
    let content =
        fs::read_to_string(path).context(format!("Failed to read file: {}", path.display()))?;

    let urls: Vec<String> = content
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(String::from)
        .collect();

    Ok(urls)
}

fn find_rss_feeds(url: &str, client: &Client) -> Result<Vec<RssFeed>> {
    // Fetch the page
    let response = client.get(url).send()?;
    let html_content = response.text()?;
    let document = Html::parse_document(&html_content);

    let mut feeds = Vec::new();

    // Look for RSS/Atom feed links in the HTML
    let link_selector =
        Selector::parse("link[type='application/rss+xml'], link[type='application/atom+xml']")
            .expect("Failed to parse CSS selector");

    for element in document.select(&link_selector) {
        if let Some(href) = element.value().attr("href") {
            let feed_url = resolve_url(url, href)?;

            // Validate the feed and get its type
            if let Some(feed_type) = validate_rss_feed(&feed_url, client) {
                let title = element
                    .value()
                    .attr("title")
                    .unwrap_or("Untitled Feed")
                    .to_string();

                feeds.push(RssFeed {
                    title,
                    url: feed_url,
                    html_url: url.to_string(),
                    feed_type,
                });
            }
        }
    }

    // If no feeds found in HTML, try common RSS feed URLs
    if feeds.is_empty() {
        let common_paths = vec![
            "/feed",
            "/rss",
            "/feed.xml",
            "/rss.xml",
            "/atom.xml",
            "/index.xml",
        ];

        for path in common_paths {
            if let Ok(feed_url) = resolve_url(url, path) {
                if let Some(feed_type) = validate_rss_feed(&feed_url, client) {
                    feeds.push(RssFeed {
                        title: extract_title_from_url(url),
                        url: feed_url,
                        html_url: url.to_string(),
                        feed_type,
                    });
                    break; // Only add the first valid common feed found
                }
            }
        }
    }

    Ok(feeds)
}

fn resolve_url(base: &str, href: &str) -> Result<String> {
    let base_url = Url::parse(base)?;
    let resolved = base_url.join(href)?;
    Ok(resolved.to_string())
}

fn validate_rss_feed(feed_url: &str, client: &Client) -> Option<FeedType> {
    // Try to fetch and parse the feed
    match client.get(feed_url).send() {
        Ok(response) => {
            if !response.status().is_success() {
                return None;
            }

            match response.text() {
                Ok(content) => {
                    // Try to parse as RSS
                    if rss::Channel::read_from(content.as_bytes()).is_ok() {
                        return Some(FeedType::Rss);
                    }

                    // Try to parse as Atom
                    if atom_syndication::Feed::read_from(content.as_bytes()).is_ok() {
                        return Some(FeedType::Atom);
                    }

                    None
                }
                Err(_) => None,
            }
        }
        Err(_) => None,
    }
}

fn extract_title_from_url(url: &str) -> String {
    Url::parse(url)
        .ok()
        .and_then(|u| u.host_str().map(String::from))
        .unwrap_or_else(|| "Unknown".to_string())
}

fn create_opml_file(feeds: &[RssFeed], output_path: &std::path::Path) -> Result<()> {
    let mut opml = opml::OPML::default();
    opml.head = Some(opml::Head {
        title: Some("RSS Feeds".to_string()),
        ..Default::default()
    });

    let mut outlines = Vec::new();

    for feed in feeds {
        let feed_type_str = match feed.feed_type {
            FeedType::Rss => "rss",
            FeedType::Atom => "atom",
        };

        let outline = opml::Outline {
            text: feed.title.clone(),
            r#type: Some(feed_type_str.to_string()),
            xml_url: Some(feed.url.clone()),
            html_url: Some(feed.html_url.clone()),
            ..Default::default()
        };
        outlines.push(outline);
    }

    opml.body = opml::Body { outlines };

    let opml_string = opml.to_string()?;
    fs::write(output_path, opml_string).context(format!(
        "Failed to write OPML file: {}",
        output_path.display()
    ))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_read_urls_from_file() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "# Comment line").unwrap();
        writeln!(temp_file, "https://example.com").unwrap();
        writeln!(temp_file).unwrap();
        writeln!(temp_file, "https://test.com").unwrap();
        writeln!(temp_file, "  https://trimmed.com  ").unwrap();

        let urls = read_urls_from_file(temp_file.path()).unwrap();
        assert_eq!(urls.len(), 3);
        assert_eq!(urls[0], "https://example.com");
        assert_eq!(urls[1], "https://test.com");
        assert_eq!(urls[2], "https://trimmed.com");
    }

    #[test]
    fn test_resolve_url_absolute() {
        let result = resolve_url("https://example.com", "https://feed.example.com/rss").unwrap();
        assert_eq!(result, "https://feed.example.com/rss");
    }

    #[test]
    fn test_resolve_url_relative() {
        let result = resolve_url("https://example.com", "/feed.xml").unwrap();
        assert_eq!(result, "https://example.com/feed.xml");
    }

    #[test]
    fn test_extract_title_from_url() {
        let title = extract_title_from_url("https://example.com/path");
        assert_eq!(title, "example.com");
    }

    #[test]
    fn test_extract_title_from_invalid_url() {
        let title = extract_title_from_url("not-a-url");
        assert_eq!(title, "Unknown");
    }

    #[test]
    fn test_create_opml_file() {
        let feeds = vec![
            RssFeed {
                title: "Test Feed 1".to_string(),
                url: "https://example.com/feed1.xml".to_string(),
                html_url: "https://example.com".to_string(),
                feed_type: FeedType::Rss,
            },
            RssFeed {
                title: "Test Feed 2".to_string(),
                url: "https://example.com/feed2.xml".to_string(),
                html_url: "https://example.com".to_string(),
                feed_type: FeedType::Atom,
            },
        ];

        let temp_file = NamedTempFile::new().unwrap();
        let output_path = temp_file.path();

        create_opml_file(&feeds, output_path).unwrap();

        let content = fs::read_to_string(output_path).unwrap();
        assert!(content.contains("Test Feed 1"));
        assert!(content.contains("Test Feed 2"));
        assert!(content.contains("https://example.com/feed1.xml"));
        assert!(content.contains("https://example.com/feed2.xml"));
        assert!(content.contains("<opml"));
    }
}
