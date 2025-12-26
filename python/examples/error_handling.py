"""
Error handling example for rss-miner.

This example demonstrates proper error handling when discovering feeds.
"""

import rss_miner


def find_with_error_handling(url):
    """Find feeds from a URL with proper error handling."""
    try:
        print(f"Processing: {url}")
        feeds = rss_miner.find_feeds(url)
        
        if feeds:
            print(f"  ✓ Found {len(feeds)} feed(s)")
            for feed in feeds:
                print(f"    • {feed.title}: {feed.url}")
        else:
            print(f"  ⚠ No feeds found")
        
        return feeds
        
    except RuntimeError as e:
        print(f"  ✗ Runtime error: {e}")
        return []
    except Exception as e:
        print(f"  ✗ Unexpected error: {e}")
        return []


def main():
    # Mix of valid and potentially problematic URLs
    urls = [
        "https://github.blog",  # Valid
        "https://stackoverflow.blog",  # Valid
        "https://example.com",  # May not have feeds
        "https://nonexistent-domain-12345.com",  # Invalid domain
        "not-a-valid-url",  # Invalid URL format
    ]
    
    print("Processing URLs with error handling...\n")
    
    all_feeds = []
    for url in urls:
        feeds = find_with_error_handling(url)
        all_feeds.extend(feeds)
        print()
    
    # Summary
    print(f"{'='*60}")
    print(f"Summary: Successfully found {len(all_feeds)} feed(s)")
    print(f"{'='*60}")
    
    if all_feeds:
        print("\nSuccessfully discovered feeds:")
        for i, feed in enumerate(all_feeds, 1):
            print(f"{i}. {feed.title}")
            print(f"   Feed: {feed.url}")
            print(f"   Site: {feed.html_url}")
        
        # Create OPML with only valid feeds
        try:
            output_file = "valid_feeds.opml"
            rss_miner.create_opml(all_feeds, output_file)
            print(f"\n✓ OPML file created: {output_file}")
        except Exception as e:
            print(f"\n✗ Failed to create OPML: {e}")


if __name__ == "__main__":
    main()
