use ressic::{Client, models::{Article, Feed}, storage::{FeedStorage, JsonLocalStorage}};
use chrono::Utc;

fn main() {
    // Load storage
    let storage = JsonLocalStorage::new("./feeds").unwrap();
    // Load client
    let mut client = Client::new(storage);
    // Load new feed
    let feed = Feed{
        name: String::from("default"),
        title: String::from("Default Feed"),
        link: String::from("https://example.com"),
        description: String::from("This is the default feed"),
        articles: vec![Article{
            title: String::from("Title"),
            content: String::from("This is some content"),
            id: String::from("1"),
            url: String::from("https://example.com/article/1"),
            summary: String::from("A brief summary"),
            pub_date: Utc::now(),
        }],
    };
    // Print it
    println!("Should work:\n {:?}", feed);
    // Store it
    client.store_article("default", feed.articles[0].clone()).unwrap();
    client.storage.set_feed_metadata("default", &feed).unwrap();
}
