//! Example demonstrating RSS 2.0 feed generation.
//!
//! This example shows how to create a feed with articles and generate
//! a valid RSS 2.0 XML document.

use chrono::{TimeZone, Utc};
use ressic::generator::{FeedGenerator, Rss20};
use ressic::models::{Article, Feed};

fn main() {
    // Create sample articles
    let articles = vec![
        Article {
            title: "First Blog Post".to_string(),
            content: "This is the full content of the first blog post.".to_string(),
            id: "post-1".to_string(),
            url: "https://blog.example.com/posts/first".to_string(),
            summary: "An introduction to our blog".to_string(),
            pub_date: Utc.with_ymd_and_hms(2026, 1, 16, 10, 30, 0).unwrap(),
        },
        Article {
            title: "Second Post: Tips & Tricks".to_string(),
            content: "Learn about <advanced> features & techniques.".to_string(),
            id: "post-2".to_string(),
            url: "https://blog.example.com/posts/second".to_string(),
            summary: "Useful tips and tricks for developers".to_string(),
            pub_date: Utc.with_ymd_and_hms(2026, 1, 16, 14, 15, 0).unwrap(),
        },
    ];

    // Create feed
    let feed = Feed {
        name: "blog".to_string(),
        title: "Example Blog".to_string(),
        link: "https://blog.example.com".to_string(),
        description: "A blog about interesting topics & ideas".to_string(),
        articles,
    };

    // Generate RSS 2.0 feed
    let generator = Rss20::new();
    match generator.generate(&feed) {
        Ok(rss) => {
            println!("Generated RSS 2.0 feed:\n");
            println!("{}", rss);
            println!("\nMIME type: {}", generator.mime_type());
            println!("Format: {}", generator.format_name());
        }
        Err(e) => eprintln!("Error generating RSS: {:?}", e),
    }
}
