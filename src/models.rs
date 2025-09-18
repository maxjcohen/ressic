#[derive(Debug, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize)]
pub struct Article {
    pub title: String,
    pub content: String,
    pub id: u32,
}