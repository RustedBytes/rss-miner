"""
Complete workflow example for rss-miner.

This example demonstrates a full workflow:
1. Reading URLs from a file
2. Finding feeds in parallel
3. Filtering and processing results
4. Creating an OPML file

Run with: python examples/complete_workflow.py
"""

import rss_miner
import tempfile
import os


def create_sample_urls_file():
    """Create a sample URLs file."""
    content = """# Technology News
https://github.blog
https://stackoverflow.blog

# Programming Languages
https://www.rust-lang.org/
https://go.dev/
https://python.org/

# Web Frameworks
https://react.dev
https://vuejs.org
"""
    
    fd, path = tempfile.mkstemp(suffix=".txt", prefix="urls_")
    with os.fdopen(fd, 'w') as f:
        f.write(content)
    return path


def main():
    print("=" * 70)
    print("RSS Miner - Complete Workflow Example")
    print("=" * 70)
    
    # Step 1: Create and read URLs from file
    print("\n📄 Step 1: Reading URLs from file")
    print("-" * 70)
    urls_file = create_sample_urls_file()
    print(f"Created temporary file: {urls_file}")
    
    try:
        urls = rss_miner.read_urls(urls_file)
        print(f"✓ Read {len(urls)} URLs from file")
        for i, url in enumerate(urls, 1):
            print(f"  {i}. {url}")
        
        # Step 2: Find feeds in parallel
        print("\n🔍 Step 2: Finding RSS/Atom feeds")
        print("-" * 70)
        print("Processing URLs in parallel (this may take a moment)...\n")
        
        feeds = rss_miner.find_feeds_parallel(urls, verbose=True)
        
        # Step 3: Process and analyze results
        print("\n📊 Step 3: Analyzing results")
        print("-" * 70)
        print(f"Total feeds discovered: {len(feeds)}")
        
        if feeds:
            # Count by feed type
            rss_count = sum(1 for f in feeds if f.feed_type == "rss")
            atom_count = sum(1 for f in feeds if f.feed_type == "atom")
            
            print(f"  • RSS feeds: {rss_count}")
            print(f"  • Atom feeds: {atom_count}")
            
            # Display all feeds
            print("\n📝 Discovered Feeds:")
            for i, feed in enumerate(feeds, 1):
                print(f"\n{i}. {feed.title}")
                print(f"   Feed URL: {feed.url}")
                print(f"   Website: {feed.html_url}")
                print(f"   Type: {feed.feed_type.upper()}")
                
                # Show dict representation
                feed_dict = feed.to_dict()
                print(f"   Dict keys: {', '.join(feed_dict.keys())}")
            
            # Step 4: Create OPML file
            print("\n💾 Step 4: Creating OPML file")
            print("-" * 70)
            output_file = "complete_workflow_feeds.opml"
            rss_miner.create_opml(feeds, output_file)
            print(f"✓ OPML file created: {output_file}")
            
            # Show file size
            if os.path.exists(output_file):
                size = os.path.getsize(output_file)
                print(f"  File size: {size} bytes")
        else:
            print("\n⚠️  No feeds were discovered")
            print("This might be due to network restrictions or the URLs not having feeds")
        
    except Exception as e:
        print(f"\n✗ Error: {e}")
        import traceback
        traceback.print_exc()
    finally:
        # Cleanup
        if os.path.exists(urls_file):
            os.remove(urls_file)
            print(f"\n🧹 Cleaned up temporary file: {urls_file}")
    
    print("\n" + "=" * 70)
    print("Workflow complete!")
    print("=" * 70)


if __name__ == "__main__":
    main()
