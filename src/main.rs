use anyhow::Result;
use clap::{Parser, ValueEnum};
use reqwest::blocking::Client;
use rss_miner::{create_opml_file_filtered, find_rss_feeds_parallel, read_urls_from_file, FeedType};
use std::path::PathBuf;

#[derive(Debug, Clone, ValueEnum)]
enum FeedFilter {
    /// Save only RSS feeds
    Rss,
    /// Save only Atom feeds
    Atom,
    /// Save both RSS and Atom feeds
    Both,
}

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

    /// Filter feeds by type (rss, atom, or both)
    #[arg(short, long, value_enum, default_value = "both")]
    filter: FeedFilter,
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
    let feeds = find_rss_feeds_parallel(&urls, &client, true);

    println!("\nTotal feeds found: {}", feeds.len());

    if feeds.is_empty() {
        println!("No RSS feeds found. OPML file will not be created.");
        return Ok(());
    }

    // Convert filter option to FeedType
    let feed_type_filter = match args.filter {
        FeedFilter::Rss => Some(FeedType::Rss),
        FeedFilter::Atom => Some(FeedType::Atom),
        FeedFilter::Both => None,
    };

    // Create OPML file with the selected filter
    create_opml_file_filtered(&feeds, &args.output, feed_type_filter)?;
    println!("OPML file created: {}", args.output.display());

    Ok(())
}

