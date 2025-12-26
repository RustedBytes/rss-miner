"""
File input example for rss-miner.

This example demonstrates how to read URLs from a text file,
process them, and create an OPML file.
"""

import rss_miner
import os
import tempfile


def create_sample_urls_file():
    """Create a sample URLs file for demonstration."""
    content = """# Tech blogs
https://github.blog
https://stackoverflow.blog

# Programming languages
https://www.rust-lang.org/
https://go.dev/

# Python
https://python.org/
"""
    
    # Create temporary file
    fd, path = tempfile.mkstemp(suffix=".txt", prefix="urls_")
    with os.fdopen(fd, 'w') as f:
        f.write(content)
    return path


def main():
    # Create a sample URLs file
    urls_file = create_sample_urls_file()
    print(f"Created sample URLs file: {urls_file}\n")
    
    try:
        # Read URLs from file
        print("Reading URLs from file...")
        urls = rss_miner.read_urls(urls_file)
        print(f"✓ Read {len(urls)} URLs\n")
        
        # Process URLs
        print("Processing URLs in parallel...\n")
        feeds = rss_miner.find_feeds_parallel(urls, verbose=True)
        
        print(f"\n{'='*60}")
        print(f"✓ Found {len(feeds)} total feed(s)")
        print(f"{'='*60}\n")
        
        # Create OPML file
        if feeds:
            output_file = "feeds_from_file.opml"
            rss_miner.create_opml(feeds, output_file)
            print(f"✓ OPML file created: {output_file}")
            
            # Display summary
            print("\nFeed Summary:")
            for feed in feeds:
                print(f"  • {feed.title} ({feed.feed_type})")
        
    except Exception as e:
        print(f"\n✗ Error: {e}")
    finally:
        # Clean up temporary file
        if os.path.exists(urls_file):
            os.remove(urls_file)
            print(f"\nCleaned up temporary file: {urls_file}")


if __name__ == "__main__":
    main()
