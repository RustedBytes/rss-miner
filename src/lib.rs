use anyhow::{Context, Result};
use rayon::prelude::*;
use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use url::Url;

#[derive(Debug, Clone)]
pub struct RssFeed {
    pub title: String,
    pub url: String,
    pub html_url: String,
    pub feed_type: FeedType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FeedType {
    Rss,
    Atom,
}

pub fn read_urls_from_file(path: &Path) -> Result<Vec<String>> {
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

pub fn find_rss_feeds(url: &str, client: &Client) -> Result<Vec<RssFeed>> {
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

pub fn find_rss_feeds_parallel(urls: &[String], client: &Client, verbose: bool) -> Vec<RssFeed> {
    urls.par_iter()
        .filter_map(|url| {
            if verbose {
                println!("Processing: {}", url);
            }
            match find_rss_feeds(url, client) {
                Ok(feeds) => {
                    if !feeds.is_empty() {
                        if verbose {
                            println!("  Found {} feed(s) for {}", feeds.len(), url);
                        }
                        Some(feeds)
                    } else {
                        if verbose {
                            println!("  No feeds found for {}", url);
                        }
                        None
                    }
                }
                Err(e) => {
                    if verbose {
                        eprintln!("  Error processing {}: {}", url, e);
                    }
                    None
                }
            }
        })
        .flatten()
        .collect()
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

pub fn create_opml_file(feeds: &[RssFeed], output_path: &Path) -> Result<()> {
    create_opml_file_filtered(feeds, output_path, None)
}

pub fn create_opml_file_filtered(
    feeds: &[RssFeed],
    output_path: &Path,
    feed_type_filter: Option<FeedType>,
) -> Result<()> {
    let mut opml = opml::OPML::default();
    
    let title = match feed_type_filter {
        Some(FeedType::Rss) => "RSS Feeds",
        Some(FeedType::Atom) => "Atom Feeds",
        None => "RSS Feeds",
    };
    
    opml.head = Some(opml::Head {
        title: Some(title.to_string()),
        ..Default::default()
    });

    let mut outlines = Vec::new();
    let mut seen_urls = HashSet::with_capacity(feeds.len());

    for feed in feeds {
        // Skip if feed doesn't match the filter
        if let Some(ref filter_type) = feed_type_filter {
            match (filter_type, &feed.feed_type) {
                (FeedType::Rss, FeedType::Rss) | (FeedType::Atom, FeedType::Atom) => {}
                _ => continue,
            }
        }

        // Skip duplicate feeds based on URL
        if seen_urls.contains(&feed.url) {
            continue;
        }
        seen_urls.insert(feed.url.clone());

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

    #[test]
    fn test_create_opml_file_with_duplicates() {
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
            RssFeed {
                title: "Test Feed 1 Duplicate".to_string(),
                url: "https://example.com/feed1.xml".to_string(), // Duplicate URL
                html_url: "https://example.com".to_string(),
                feed_type: FeedType::Rss,
            },
            RssFeed {
                title: "Test Feed 3".to_string(),
                url: "https://example.com/feed3.xml".to_string(),
                html_url: "https://example.com".to_string(),
                feed_type: FeedType::Rss,
            },
            RssFeed {
                title: "Test Feed 2 Duplicate".to_string(),
                url: "https://example.com/feed2.xml".to_string(), // Duplicate URL
                html_url: "https://example.com".to_string(),
                feed_type: FeedType::Atom,
            },
        ];

        let temp_file = NamedTempFile::new().unwrap();
        let output_path = temp_file.path();

        create_opml_file(&feeds, output_path).unwrap();

        let content = fs::read_to_string(output_path).unwrap();
        
        // Should contain first occurrence of each feed
        assert!(content.contains("Test Feed 1"));
        assert!(content.contains("Test Feed 2"));
        assert!(content.contains("Test Feed 3"));
        
        // Should NOT contain duplicate titles
        assert!(!content.contains("Test Feed 1 Duplicate"));
        assert!(!content.contains("Test Feed 2 Duplicate"));
        
        // Count occurrences of each URL - should appear only once
        assert_eq!(content.matches("https://example.com/feed1.xml").count(), 1);
        assert_eq!(content.matches("https://example.com/feed2.xml").count(), 1);
        assert_eq!(content.matches("https://example.com/feed3.xml").count(), 1);
    }

    #[test]
    fn test_create_opml_file_rss_only() {
        let feeds = vec![
            RssFeed {
                title: "RSS Feed 1".to_string(),
                url: "https://example.com/rss1.xml".to_string(),
                html_url: "https://example.com".to_string(),
                feed_type: FeedType::Rss,
            },
            RssFeed {
                title: "Atom Feed 1".to_string(),
                url: "https://example.com/atom1.xml".to_string(),
                html_url: "https://example.com".to_string(),
                feed_type: FeedType::Atom,
            },
            RssFeed {
                title: "RSS Feed 2".to_string(),
                url: "https://example.com/rss2.xml".to_string(),
                html_url: "https://example.com".to_string(),
                feed_type: FeedType::Rss,
            },
        ];

        let temp_file = NamedTempFile::new().unwrap();
        let output_path = temp_file.path();

        create_opml_file_filtered(&feeds, output_path, Some(FeedType::Rss)).unwrap();

        let content = fs::read_to_string(output_path).unwrap();

        // Should contain RSS feeds only
        assert!(content.contains("RSS Feed 1"));
        assert!(content.contains("RSS Feed 2"));
        assert!(content.contains("https://example.com/rss1.xml"));
        assert!(content.contains("https://example.com/rss2.xml"));
        
        // Should NOT contain Atom feeds
        assert!(!content.contains("Atom Feed 1"));
        assert!(!content.contains("https://example.com/atom1.xml"));
        
        // Should have appropriate title
        assert!(content.contains("RSS Feeds"));
    }

    #[test]
    fn test_create_opml_file_atom_only() {
        let feeds = vec![
            RssFeed {
                title: "RSS Feed 1".to_string(),
                url: "https://example.com/rss1.xml".to_string(),
                html_url: "https://example.com".to_string(),
                feed_type: FeedType::Rss,
            },
            RssFeed {
                title: "Atom Feed 1".to_string(),
                url: "https://example.com/atom1.xml".to_string(),
                html_url: "https://example.com".to_string(),
                feed_type: FeedType::Atom,
            },
            RssFeed {
                title: "Atom Feed 2".to_string(),
                url: "https://example.com/atom2.xml".to_string(),
                html_url: "https://example.com".to_string(),
                feed_type: FeedType::Atom,
            },
        ];

        let temp_file = NamedTempFile::new().unwrap();
        let output_path = temp_file.path();

        create_opml_file_filtered(&feeds, output_path, Some(FeedType::Atom)).unwrap();

        let content = fs::read_to_string(output_path).unwrap();

        // Should contain Atom feeds only
        assert!(content.contains("Atom Feed 1"));
        assert!(content.contains("Atom Feed 2"));
        assert!(content.contains("https://example.com/atom1.xml"));
        assert!(content.contains("https://example.com/atom2.xml"));
        
        // Should NOT contain RSS feeds
        assert!(!content.contains("RSS Feed 1"));
        assert!(!content.contains("https://example.com/rss1.xml"));
        
        // Should have appropriate title
        assert!(content.contains("Atom Feeds"));
    }
}

// Python bindings module
#[cfg(feature = "python")]
pub mod python {
    use super::*;
    use pyo3::prelude::*;
    use std::collections::HashMap;

    /// Python wrapper for RssFeed
    #[pyclass]
    #[derive(Clone)]
    pub struct PyRssFeed {
        #[pyo3(get)]
        pub title: String,
        #[pyo3(get)]
        pub url: String,
        #[pyo3(get)]
        pub html_url: String,
        #[pyo3(get)]
        pub feed_type: String,
    }

    impl From<RssFeed> for PyRssFeed {
        fn from(feed: RssFeed) -> Self {
            PyRssFeed {
                title: feed.title,
                url: feed.url,
                html_url: feed.html_url,
                feed_type: match feed.feed_type {
                    FeedType::Rss => "rss".to_string(),
                    FeedType::Atom => "atom".to_string(),
                },
            }
        }
    }

    #[pymethods]
    impl PyRssFeed {
        fn __repr__(&self) -> String {
            format!(
                "RssFeed(title='{}', url='{}', html_url='{}', feed_type='{}')",
                self.title, self.url, self.html_url, self.feed_type
            )
        }

        fn to_dict(&self) -> HashMap<String, String> {
            let mut map = HashMap::new();
            map.insert("title".to_string(), self.title.clone());
            map.insert("url".to_string(), self.url.clone());
            map.insert("html_url".to_string(), self.html_url.clone());
            map.insert("feed_type".to_string(), self.feed_type.clone());
            map
        }
    }

    /// Find RSS/Atom feeds from a single URL
    #[pyfunction]
    fn find_feeds(url: String) -> PyResult<Vec<PyRssFeed>> {
        let client = Client::new();
        let feeds = find_rss_feeds(&url, &client)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        Ok(feeds.into_iter().map(PyRssFeed::from).collect())
    }

    /// Find RSS/Atom feeds from multiple URLs in parallel
    #[pyfunction]
    #[pyo3(signature = (urls, verbose=false))]
    fn find_feeds_parallel(urls: Vec<String>, verbose: bool) -> PyResult<Vec<PyRssFeed>> {
        let client = Client::new();
        let feeds = find_rss_feeds_parallel(&urls, &client, verbose);
        Ok(feeds.into_iter().map(PyRssFeed::from).collect())
    }

    /// Read URLs from a text file
    #[pyfunction]
    fn read_urls(file_path: String) -> PyResult<Vec<String>> {
        let path = Path::new(&file_path);
        read_urls_from_file(path)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("{}", e)))
    }

    /// Create an OPML file from a list of feeds
    #[pyfunction]
    fn create_opml(feeds: Vec<PyRssFeed>, output_path: String) -> PyResult<()> {
        let rust_feeds: Vec<RssFeed> = feeds
            .into_iter()
            .map(|py_feed| RssFeed {
                title: py_feed.title,
                url: py_feed.url,
                html_url: py_feed.html_url,
                feed_type: if py_feed.feed_type == "rss" {
                    FeedType::Rss
                } else {
                    FeedType::Atom
                },
            })
            .collect();

        let path = Path::new(&output_path);
        create_opml_file(&rust_feeds, path)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("{}", e)))
    }

    /// Create an OPML file containing only RSS feeds
    #[pyfunction]
    fn create_opml_rss_only(feeds: Vec<PyRssFeed>, output_path: String) -> PyResult<()> {
        let rust_feeds: Vec<RssFeed> = feeds
            .into_iter()
            .map(|py_feed| RssFeed {
                title: py_feed.title,
                url: py_feed.url,
                html_url: py_feed.html_url,
                feed_type: if py_feed.feed_type == "rss" {
                    FeedType::Rss
                } else {
                    FeedType::Atom
                },
            })
            .collect();

        let path = Path::new(&output_path);
        create_opml_file_filtered(&rust_feeds, path, Some(FeedType::Rss))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("{}", e)))
    }

    /// Create an OPML file containing only Atom feeds
    #[pyfunction]
    fn create_opml_atom_only(feeds: Vec<PyRssFeed>, output_path: String) -> PyResult<()> {
        let rust_feeds: Vec<RssFeed> = feeds
            .into_iter()
            .map(|py_feed| RssFeed {
                title: py_feed.title,
                url: py_feed.url,
                html_url: py_feed.html_url,
                feed_type: if py_feed.feed_type == "rss" {
                    FeedType::Rss
                } else {
                    FeedType::Atom
                },
            })
            .collect();

        let path = Path::new(&output_path);
        create_opml_file_filtered(&rust_feeds, path, Some(FeedType::Atom))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("{}", e)))
    }

    /// RSS Miner Python module
    #[pymodule]
    fn rss_miner(m: &Bound<'_, PyModule>) -> PyResult<()> {
        m.add_class::<PyRssFeed>()?;
        m.add_function(wrap_pyfunction!(find_feeds, m)?)?;
        m.add_function(wrap_pyfunction!(find_feeds_parallel, m)?)?;
        m.add_function(wrap_pyfunction!(read_urls, m)?)?;
        m.add_function(wrap_pyfunction!(create_opml, m)?)?;
        m.add_function(wrap_pyfunction!(create_opml_rss_only, m)?)?;
        m.add_function(wrap_pyfunction!(create_opml_atom_only, m)?)?;
        Ok(())
    }
}
