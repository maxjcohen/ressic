#[derive(Debug, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize)]
pub struct Article {
    pub title: String,
    pub content: String,
    pub id: u32,
}

impl Article {
    pub fn new_empty() -> Self {
        Self {
            title: String::from(""),
            content: String::from(""),
            id: 0,
        }
    }
}
