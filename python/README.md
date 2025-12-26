# rss-miner Python Bindings

High-performance Python bindings for the rss-miner Rust library. Discover RSS/Atom feeds from URLs and generate OPML files with ease.

## Features

- 🚀 **Fast**: Built on top of Rust for maximum performance
- 🔍 **Parallel Processing**: Process multiple URLs concurrently using Rayon
- ✅ **Feed Validation**: Validates RSS/Atom feeds before returning them
- 📝 **OPML Generation**: Create valid OPML files for feed readers
- 🎯 **Auto-Discovery**: Finds feeds in HTML link tags and common paths
- 🐍 **Pythonic API**: Clean, ergonomic Python interface
- 🛡️ **Type Safe**: Full type hints for better IDE support

## Requirements

- Python 3.10 or higher
- [uv](https://github.com/astral-sh/uv) (recommended) or Python's built-in venv for development

## Installation

### Using pip (when published to PyPI)

```bash
pip install rss-miner
```

### Development Setup

#### Option 1: Using uv (recommended)

```bash
# Install uv if you haven't already
curl -LsSf https://astral.sh/uv/install.sh | sh

# Create a virtual environment
uv venv

# Activate the virtual environment
source .venv/bin/activate  # On Unix/macOS
# or
.venv\Scripts\activate  # On Windows

# Install maturin
uv pip install maturin

# Build and install the package in development mode
maturin develop --release
```

#### Option 2: Using Python venv

```bash
# Create a virtual environment
python3 -m venv .venv

# Activate the virtual environment
source .venv/bin/activate  # On Unix/macOS
# or
.venv\Scripts\activate  # On Windows

# Install maturin
pip install maturin

# Build and install the package in development mode
maturin develop --release
```

### Building from source

```bash
# After setting up your virtual environment with either uv or venv
# Install maturin if not already installed
pip install maturin

# Build and install
maturin develop --release
```

## Quick Start

```python
import rss_miner

# Find feeds from a single URL
feeds = rss_miner.find_feeds("https://github.blog")
for feed in feeds:
    print(f"Found: {feed.title} - {feed.url}")

# Find feeds from multiple URLs in parallel
urls = [
    "https://github.blog",
    "https://stackoverflow.blog",
    "https://www.rust-lang.org/"
]
feeds = rss_miner.find_feeds_parallel(urls, verbose=True)

# Create an OPML file
rss_miner.create_opml(feeds, "feeds.opml")
```

## API Reference

### Functions

#### `find_feeds(url: str) -> list[RssFeed]`

Find RSS/Atom feeds from a single URL.

**Parameters:**
- `url` (str): The URL to search for feeds

**Returns:**
- List of `RssFeed` objects found at the URL

**Example:**
```python
feeds = rss_miner.find_feeds("https://github.blog")
```

#### `find_feeds_parallel(urls: list[str], verbose: bool = False) -> list[RssFeed]`

Find RSS/Atom feeds from multiple URLs in parallel.

**Parameters:**
- `urls` (list[str]): List of URLs to search for feeds
- `verbose` (bool, optional): Enable verbose output. Defaults to False.

**Returns:**
- List of all `RssFeed` objects found across all URLs

**Example:**
```python
urls = ["https://github.blog", "https://stackoverflow.blog"]
feeds = rss_miner.find_feeds_parallel(urls, verbose=True)
```

#### `read_urls(file_path: str) -> list[str]`

Read URLs from a text file.

**Parameters:**
- `file_path` (str): Path to the text file containing URLs (one per line)

**Returns:**
- List of URLs read from the file

**Notes:**
- Lines starting with `#` are treated as comments
- Empty lines are ignored
- Leading/trailing whitespace is trimmed

**Example:**
```python
urls = rss_miner.read_urls("urls.txt")
```

#### `create_opml(feeds: list[RssFeed], output_path: str) -> None`

Create an OPML file from a list of feeds.

**Parameters:**
- `feeds` (list[RssFeed]): List of RssFeed objects to include in the OPML
- `output_path` (str): Path where the OPML file will be saved

**Example:**
```python
rss_miner.create_opml(feeds, "my_feeds.opml")
```

### Classes

#### `RssFeed`

Represents a discovered RSS or Atom feed.

**Attributes:**
- `title` (str): The title of the feed
- `url` (str): The URL of the feed
- `html_url` (str): The URL of the website
- `feed_type` (str): The type of feed ("rss" or "atom")

**Methods:**
- `to_dict()`: Convert the feed to a dictionary
- `__repr__()`: String representation of the feed

**Example:**
```python
feed = feeds[0]
print(f"Title: {feed.title}")
print(f"Feed URL: {feed.url}")
print(f"Website: {feed.html_url}")
print(f"Type: {feed.feed_type}")

# Convert to dict
feed_dict = feed.to_dict()
```

## Examples

See the [examples](./examples/) directory for more usage examples:

- [basic_usage.py](./examples/basic_usage.py) - Basic feed discovery
- [batch_processing.py](./examples/batch_processing.py) - Process multiple URLs
- [file_input.py](./examples/file_input.py) - Read URLs from file
- [error_handling.py](./examples/error_handling.py) - Handle errors gracefully

## Development

### Setup Development Environment

```bash
# Clone the repository
git clone https://github.com/RustedBytes/rss-miner.git
cd rss-miner/python

# Option 1: Using uv (recommended)
curl -LsSf https://astral.sh/uv/install.sh | sh
uv venv
source .venv/bin/activate  # Unix/macOS
uv pip install maturin

# Option 2: Using Python venv
python3 -m venv .venv
source .venv/bin/activate  # Unix/macOS
pip install maturin

# Build the extension in development mode
maturin develop --release
```

### Running Tests

```bash
# Run the basic test script
python test_basic.py

# Or run with pytest (if installed)
pip install pytest
pytest

# Run Rust tests
cd ..
cargo test --features python
```
cargo test --features python
```

### Building Wheels

```bash
# Build wheel for current platform
maturin build --release

# Build wheels for multiple platforms (requires Docker)
maturin build --release --manylinux 2014
```

## Performance

The Python bindings leverage Rust's performance and safety:

- **Parallel Processing**: Utilizes all CPU cores for concurrent URL processing
- **Efficient Memory**: Rust's ownership system ensures minimal memory overhead
- **Fast HTTP**: Uses the battle-tested `reqwest` library
- **Zero-Copy**: Minimal data copying between Rust and Python

## License

This project is dual-licensed under MIT OR Apache-2.0. See the LICENSE file in the repository root for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Links

- [GitHub Repository](https://github.com/RustedBytes/rss-miner)
- [Issue Tracker](https://github.com/RustedBytes/rss-miner/issues)
- [PyPI Package](https://pypi.org/project/rss-miner/) (when published)
