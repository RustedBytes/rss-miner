use anyhow::Result;
use clap::Parser;
use reqwest::blocking::Client;
use rss_miner::{create_opml_file, find_rss_feeds_parallel, read_urls_from_file};
use std::path::PathBuf;

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

    // Create OPML file
    create_opml_file(&feeds, &args.output)?;
    println!("OPML file created: {}", args.output.display());

    Ok(())
}

