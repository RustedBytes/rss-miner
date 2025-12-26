"""
rss-miner: A fast RSS/Atom feed discovery and OPML generation tool.

This package provides Python bindings for the rss-miner Rust library,
offering high-performance RSS/Atom feed discovery and OPML file generation.
"""

from .rss_miner import (
    find_feeds,
    find_feeds_parallel,
    read_urls,
    create_opml,
    PyRssFeed as RssFeed,
)

__all__ = [
    "find_feeds",
    "find_feeds_parallel",
    "read_urls",
    "create_opml",
    "RssFeed",
]

__version__ = "0.1.0"
