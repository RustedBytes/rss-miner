"""
Separate feed types example for rss-miner.

This example demonstrates how to save RSS and Atom feeds separately
using the new create_opml_rss_only and create_opml_atom_only functions.

Run with: python examples/separate_feed_types.py
"""

import rss_miner


def main():
    print("=" * 70)
    print("RSS Miner - Separate Feed Types Example")
    print("=" * 70)
    
    # Sample URLs to demonstrate different feed types
    urls = [
        "https://github.blog",
        "https://stackoverflow.blog",
        "https://www.rust-lang.org/",
    ]
    
    print("\n🔍 Step 1: Finding feeds from URLs")
    print("-" * 70)
    feeds = rss_miner.find_feeds_parallel(urls, verbose=True)
    
    if not feeds:
        print("\n⚠️  No feeds were discovered")
        print("This might be due to network restrictions or the URLs not having feeds")
        return
    
    print(f"\n📊 Step 2: Analyzing feed types")
    print("-" * 70)
    print(f"Total feeds discovered: {len(feeds)}")
    
    # Count by feed type
    rss_feeds = [f for f in feeds if f.feed_type == "rss"]
    atom_feeds = [f for f in feeds if f.feed_type == "atom"]
    
    print(f"  • RSS feeds: {len(rss_feeds)}")
    print(f"  • Atom feeds: {len(atom_feeds)}")
    
    # Display all feeds
    print("\n📝 All Discovered Feeds:")
    for i, feed in enumerate(feeds, 1):
        print(f"\n{i}. {feed.title}")
        print(f"   Type: {feed.feed_type.upper()}")
        print(f"   Feed URL: {feed.url}")
        print(f"   Website: {feed.html_url}")
    
    # Step 3: Create separate OPML files
    print("\n💾 Step 3: Creating separate OPML files")
    print("-" * 70)
    
    # Create OPML with all feeds
    all_feeds_file = "all_feeds.opml"
    rss_miner.create_opml(feeds, all_feeds_file)
    print(f"✓ All feeds saved to: {all_feeds_file} ({len(feeds)} feeds)")
    
    # Create OPML with RSS feeds only
    if rss_feeds:
        rss_only_file = "rss_feeds_only.opml"
        rss_miner.create_opml_rss_only(feeds, rss_only_file)
        print(f"✓ RSS feeds saved to: {rss_only_file} ({len(rss_feeds)} feeds)")
    else:
        print("⚠️  No RSS feeds to save")
    
    # Create OPML with Atom feeds only
    if atom_feeds:
        atom_only_file = "atom_feeds_only.opml"
        rss_miner.create_opml_atom_only(feeds, atom_only_file)
        print(f"✓ Atom feeds saved to: {atom_only_file} ({len(atom_feeds)} feeds)")
    else:
        print("⚠️  No Atom feeds to save")
    
    print("\n" + "=" * 70)
    print("Example complete!")
    print("=" * 70)
    print("\nYou can import the generated OPML files into your favorite RSS reader.")


if __name__ == "__main__":
    main()
