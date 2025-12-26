"""
Simple test script for rss-miner Python bindings.

Run with: python test_basic.py
"""

import tempfile
import os
import sys


def test_imports():
    """Test that all expected functions and classes can be imported."""
    import rss_miner
    
    assert hasattr(rss_miner, 'find_feeds')
    assert hasattr(rss_miner, 'find_feeds_parallel')
    assert hasattr(rss_miner, 'read_urls')
    assert hasattr(rss_miner, 'create_opml')
    assert hasattr(rss_miner, 'RssFeed')
    assert hasattr(rss_miner, 'PyRssFeed')
    assert rss_miner.RssFeed is rss_miner.PyRssFeed
    print("✓ Import test passed")


def test_read_urls():
    """Test reading URLs from a file."""
    import rss_miner
    
    # Create temp file
    fd, path = tempfile.mkstemp(suffix=".txt")
    try:
        with os.fdopen(fd, 'w') as f:
            f.write("# Comment\n")
            f.write("https://example.com\n")
            f.write("\n")
            f.write("https://test.org\n")
            f.write("  https://trimmed.com  \n")
        
        urls = rss_miner.read_urls(path)
        assert len(urls) == 3
        assert urls[0] == "https://example.com"
        assert urls[1] == "https://test.org"
        assert urls[2] == "https://trimmed.com"
        print("✓ read_urls test passed")
    finally:
        os.remove(path)


def test_version():
    """Test that version is accessible."""
    import rss_miner
    
    assert hasattr(rss_miner, '__version__')
    assert isinstance(rss_miner.__version__, str)
    assert len(rss_miner.__version__) > 0
    print(f"✓ Version test passed (version: {rss_miner.__version__})")


def main():
    print("Running rss-miner Python binding tests...\n")
    
    try:
        test_imports()
        test_read_urls()
        test_version()
        
        print("\n" + "=" * 60)
        print("All tests passed! ✓")
        print("=" * 60)
        return 0
    except AssertionError as e:
        print(f"\n✗ Test failed: {e}")
        return 1
    except Exception as e:
        print(f"\n✗ Unexpected error: {e}")
        import traceback
        traceback.print_exc()
        return 1


if __name__ == "__main__":
    sys.exit(main())
