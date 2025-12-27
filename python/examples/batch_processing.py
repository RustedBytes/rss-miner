"""
Batch processing example for rss-miner.

This example demonstrates how to process multiple URLs in parallel
and generate an OPML file.
"""

import rss_miner


def main():
    # List of URLs to process
    urls = [
        "https://github.blog",
        "https://stackoverflow.blog",
        "https://www.rust-lang.org/",
        "https://go.dev/",
        "https://python.org/",
    ]

    print(f"Processing {len(urls)} URLs in parallel...\n")

    try:
        # Find feeds from all URLs in parallel with verbose output
        feeds, statuses = rss_miner.find_feeds_parallel(urls, verbose=True)
        failed = [url for url, status in statuses if status == "failed"]
        if failed:
            print(f"\n⚠️  {len(failed)} URL(s) failed:")
            for failed_url in failed:
                print(f"  - {failed_url}")

        print(f"\n{'=' * 60}")
        print(f"✓ Found {len(feeds)} total feed(s)")
        print(f"{'=' * 60}\n")

        # Display all discovered feeds
        for i, feed in enumerate(feeds, 1):
            print(f"{i}. {feed.title}")
            print(f"   {feed.url}")
            print(f"   Type: {feed.feed_type}")
            print()

        # Create OPML file
        if feeds:
            output_file = "discovered_feeds.opml"
            rss_miner.create_opml(feeds, output_file)
            print(f"✓ OPML file created: {output_file}")

    except Exception as e:
        print(f"\n✗ Error: {e}")


if __name__ == "__main__":
    main()
