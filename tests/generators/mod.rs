use chrono::{TimeZone, Utc};
use ressic::generator::FeedGenerator;
use ressic::models::{Article, Feed};

mod plain_text_tests;
mod rss20_tests;

pub fn test_generate<G: FeedGenerator>(generator: &G) {
    let feed = Feed::new(
        "test-feed".into(),
        "Test Feed Title".into(),
        "https://test.com".into(),
        "A feed for testing".into(),
        vec![
            Article::new(
                "Test Article".into(),
                "Content".into(),
                "1".into(),
                "https://test.com/1".into(),
                "Summary".into(),
                Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap(),
            )
            .unwrap(),
            Article::new(
                "Test Article 2".into(),
                "Content".into(),
                "2".into(),
                "https://test.com/2".into(),
                "Summary".into(),
                Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap(),
            )
            .unwrap(),
        ],
    )
    .unwrap();

    assert!(generator.generate(&feed).is_ok());
}
