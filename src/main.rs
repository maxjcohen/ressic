use chrono::Utc;
use ressic::{
    Client,
    generator::{FeedGenerator, PlainText},
    models::{Article, Feed},
    storage::{FeedStorage, JsonLocalStorage},
};

fn main() {
    // Load storage and generator
    let storage = JsonLocalStorage::new("./feeds").unwrap();
    let generator = PlainText::new();
    // Load client
    let mut client = Client::new(storage, generator);
    // Load new feed
    let feed = Feed {
        name: String::from("default"),
        title: String::from("Default Feed"),
        link: String::from("https://example.com"),
        description: String::from("This is the default feed"),
        articles: vec![Article {
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
    client
        .store_article("default", feed.articles[0].clone())
        .unwrap();
    client.storage.set_feed_metadata("default", &feed).unwrap();
    // Define a new article
    let new_article = Article {
        title: String::from("Another Title"),
        content: String::from("This is some more content"),
        id: String::from("2"),
        url: String::from("https://example.com/article/2"),
        summary: String::from("Another brief summary"),
        pub_date: Utc::now(),
    };
    // Store the new article
    client.store_article("default", new_article).unwrap();
    // Generate a feed content from this feed
    print!(
        "{}",
        client
            .generator
            .generate(&client.storage.get_feed("default").unwrap())
            .unwrap()
    );
}
