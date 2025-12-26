"""
Basic usage example for rss-miner.

This example demonstrates how to find RSS/Atom feeds from a single URL.
"""

import rss_miner


def main():
    # Find feeds from a single URL
    print("Finding RSS/Atom feeds from https://github.blog...")
    
    try:
        feeds = rss_miner.find_feeds("https://github.blog")
        
        if feeds:
            print(f"\n✓ Found {len(feeds)} feed(s):\n")
            for i, feed in enumerate(feeds, 1):
                print(f"{i}. {feed.title}")
                print(f"   Feed URL: {feed.url}")
                print(f"   Website: {feed.html_url}")
                print(f"   Type: {feed.feed_type}")
                print()
        else:
            print("\n✗ No feeds found")
            
    except Exception as e:
        print(f"\n✗ Error: {e}")


if __name__ == "__main__":
    main()
