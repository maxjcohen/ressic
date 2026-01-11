use super::{FeedStorage, StorageError};
use crate::models::Article;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

/// Local file-based storage implementation using JSONL format.
///
/// Each feed is stored as a separate `.jsonl` file where each line
/// contains a JSON-serialized article. This provides simple, portable
/// storage without external dependencies.
pub struct JsonLocalStorage {
    base_dir: PathBuf,
}

impl JsonLocalStorage {
    /// Creates a new local storage instance.
    ///
    /// The base directory will be created if it doesn't exist.
    ///
    /// # Arguments
    ///
    /// * `base_dir` - The directory where feed files will be stored
    ///
    /// # Errors
    ///
    /// Returns `StorageError::Io` if the directory cannot be created.
    pub fn new<P: AsRef<Path>>(base_dir: P) -> Result<Self, StorageError> {
        let p = base_dir.as_ref().to_path_buf();
        if !p.exists() {
            std::fs::create_dir_all(&p)?;
        }
        Ok(JsonLocalStorage { base_dir: p })
    }

    fn feed_path(&self, feed: &str) -> PathBuf {
        let mut p = self.base_dir.clone();
        p.push(format!("{}.jsonl", feed));
        p
    }

    fn read_all(&self, feed: &str) -> Result<Vec<Article>, StorageError> {
        let path = self.feed_path(feed);
        if !path.exists() {
            return Ok(vec![]);
        }

        let f = File::open(&path)?;
        let reader = BufReader::new(f);
        let mut out = Vec::new();
        for line in reader.lines() {
            let l = line?;
            if l.trim().is_empty() {
                continue;
            }
            let a: Article = serde_json::from_str(&l)?;
            out.push(a);
        }
        Ok(out)
    }

    fn write_all(&self, feed: &str, articles: &[Article]) -> Result<(), StorageError> {
        let path = self.feed_path(feed);
        let tmp_path = path.with_extension("jsonl.tmp");

        if let Some(parent) = tmp_path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        }

        let tmpf = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&tmp_path)?;
        let mut writer = BufWriter::new(tmpf);
        for a in articles {
            let line = serde_json::to_string(a)?;
            writer.write_all(line.as_bytes())?;
            writer.write_all(b"\n")?;
        }
        writer.flush()?;

        // Atomically replace
        std::fs::rename(tmp_path, path)?;
        Ok(())
    }
}

impl FeedStorage for JsonLocalStorage {
    fn get_all_articles(&self, feed: &str) -> Result<Vec<Article>, StorageError> {
        let articles = self.read_all(feed)?;
        // Return in insertion order
        Ok(articles)
    }

    fn get_latest_article(&self, feed: &str) -> Result<Article, StorageError> {
        let articles = self.read_all(feed)?;
        articles
            .into_iter()
            .last()
            .ok_or(StorageError::FeedEmpty)
    }

    fn store_article(&self, feed: &str, article: Article) -> Result<(), StorageError> {
        // Read existing, dedupe by id, update or append, then rewrite file
        let mut articles = self.read_all(feed)?;
        if let Some(a) = articles.iter_mut().find(|a| a.id == article.id) {
            *a = article;
        } else {
            articles.push(article);
        }
        self.write_all(feed, &articles)
    }
}