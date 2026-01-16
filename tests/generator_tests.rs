use chrono::{TimeZone, Utc};
use ressic::generator::{FeedGenerator, PlainText};
use ressic::models::{Article, Feed};

#[test]
fn test_generator() {
    fn test_generate<G: FeedGenerator>(
        generator: &G,
        feed: &Feed,
    ) -> Result<String, ressic::generator::GeneratorError> {
        generator.generate(feed)
    }

    let feed = Feed {
        name: "Test Feed".into(),
        title: "Test Feed Title".into(),
        link: "https://test.com".into(),
        description: "A feed for testing".into(),
        articles: vec![
            Article {
                title: "Test Article".into(),
                content: "Content".into(),
                id: "1".into(),
                url: "https://test.com/1".into(),
                summary: "Summary".into(),
                pub_date: Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap(),
            },
            Article {
                title: "Test Article 2".into(),
                content: "Content".into(),
                id: "2".into(),
                url: "https://test.com/2".into(),
                summary: "Summary".into(),
                pub_date: Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap(),
            },
        ],
    };

    assert!(test_generate(&PlainText::new(), &feed).is_ok());
}
