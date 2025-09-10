use ressic::{models::Article, storage::LocalFile, Client};

fn main() {
    // Load storage
    let storage = LocalFile::new("./feeds").unwrap();
    // Load client
    let mut client = Client::new(storage);
    // Load new article
    let article = Article{
            title: String::from("Title"),
            content: String::from("This is some content"),
            id: 1,
    };
    // Print it
    println!("Should work:\n {:?}", article);
    // Store it
    client.store_article("default", article).unwrap();
}
