use std::path::Path;

fn main() {
    let feeds = vec![
        rss_miner::RssFeed {
            title: "Test Feed 1".to_string(),
            url: "https://example.com/feed1.xml".to_string(),
            html_url: "https://example.com".to_string(),
            feed_type: rss_miner::FeedType::Rss,
        },
        rss_miner::RssFeed {
            title: "Test Feed 2".to_string(),
            url: "https://example.com/feed2.xml".to_string(),
            html_url: "https://example.com".to_string(),
            feed_type: rss_miner::FeedType::Atom,
        },
    ];

    let output_path = Path::new("/tmp/manual_test.opml");
    rss_miner::create_opml_file(&feeds, output_path).unwrap();
    
    let content = std::fs::read_to_string(output_path).unwrap();
    println!("{}", content);
}
