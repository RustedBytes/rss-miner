# rss-miner

[![Crates.io Version](https://img.shields.io/crates/v/rss-miner)](https://crates.io/crates/rss-miner)
[![MIT licensed](https://img.shields.io/badge/license-MIT-yellow.svg)](https://github.com/RustedBytes/rss-miner/blob/master/LICENSE)

A CLI tool that finds RSS feeds from URLs and generates a valid OPML file.

## Features

- **Parallel Processing**: Uses Rayon to process multiple URLs concurrently
- **RSS Feed Validation**: Validates RSS/Atom feeds before including them
- **OPML Generation**: Creates a valid OPML file compatible with feed readers
- **Auto-Discovery**: Finds RSS feeds in HTML link tags and common feed paths
- **Error Handling**: Robust error handling with detailed feedback

## Installation

```bash
cargo build --release
```

## Usage

```bash
rss-miner --input <INPUT_FILE> [--output <OUTPUT_FILE>]
```

### Arguments

- `-i, --input <FILE>`: Input file containing URLs (one per line, required)
- `-o, --output <FILE>`: Output OPML file path (default: `feeds.opml`)

### Example

Create a file `urls.txt` with URLs:

```
https://github.blog
https://stackoverflow.blog
https://www.rust-lang.org/
```

Run the command:

```bash
cargo run -- --input urls.txt --output feeds.opml
```

Or use the compiled binary:

```bash
./target/release/rss-miner --input urls.txt --output feeds.opml
```

### Input File Format

- One URL per line
- Lines starting with `#` are treated as comments and ignored
- Empty lines are ignored

Example:

```
# Tech blogs
https://github.blog
https://stackoverflow.blog

# Programming languages
https://www.rust-lang.org/
https://go.dev/
```

## How It Works

1. **Reads URLs**: Parses the input file to extract URLs
2. **Parallel Processing**: Uses Rayon to process multiple URLs simultaneously
3. **Feed Discovery**: For each URL:
   - Fetches the HTML page
   - Looks for RSS/Atom feed links in the HTML
   - Checks common RSS feed paths (`/feed`, `/rss`, `/feed.xml`, etc.)
4. **Validation**: Validates each discovered feed by:
   - Attempting to fetch the feed
   - Parsing it as RSS or Atom format
5. **OPML Generation**: Creates a valid OPML file with all discovered and validated feeds

## Dependencies

- **clap**: Command-line argument parsing
- **rayon**: Parallel processing
- **reqwest**: HTTP client
- **scraper**: HTML parsing
- **opml**: OPML file generation
- **rss**: RSS feed parsing and validation
- **atom_syndication**: Atom feed parsing and validation

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

