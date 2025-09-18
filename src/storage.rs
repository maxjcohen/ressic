use super::models::Article;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum StorageError {
    Io(std::io::Error),
    Json(serde_json::Error),
    FeedEmpty,
}

impl From<std::io::Error> for StorageError {
    fn from(e: std::io::Error) -> Self {
        StorageError::Io(e)
    }
}

impl From<serde_json::Error> for StorageError {
    fn from(e: serde_json::Error) -> Self {
        StorageError::Json(e)
    }
}

pub trait FeedStorage {
    fn get_all_articles(&self, feed: &str) -> Result<Vec<Article>, StorageError>;
    fn get_latest_article(&self, feed: &str) -> Result<Article, StorageError>;
    fn store_article(&mut self, feed: &str, article: Article) -> Result<(), StorageError>;
}

pub struct LocalFile {
    base_dir: PathBuf,
}

impl LocalFile {
    pub fn new<P: AsRef<Path>>(base_dir: P) -> Result<Self, StorageError> {
        let p = base_dir.as_ref().to_path_buf();
        if !p.exists() {
            std::fs::create_dir_all(&p)?;
        }
        Ok(LocalFile { base_dir: p })
    }

    fn feed_path(&self, feed: &str) -> PathBuf {
        let mut p = self.base_dir.clone();
        p.push(format!("{}.jsonl", feed));
        p
    }

    // Advisory lock implementation: we'll open the file for append/read and rely on OS-level advisory behavior.
    // Since we are avoiding extra deps, we won't enforce locking beyond opening with append when writing.
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

impl FeedStorage for LocalFile {
    fn get_all_articles(&self, feed: &str) -> Result<Vec<Article>, StorageError> {
        let articles = self.read_all(feed)?;
        // Return in insertion order
        Ok(articles)
    }

    fn get_latest_article(&self, feed: &str) -> Result<Article, StorageError> {
        let articles = self.read_all(feed)?;
        if let Some(last) = articles.last() {
            Ok(last.clone())
        } else {
            Err(StorageError::FeedEmpty)
        }
    }

    fn store_article(&mut self, feed: &str, article: Article) -> Result<(), StorageError> {
        // Read existing, dedupe by id, update or append, then rewrite file
        let mut articles = self.read_all(feed)?;
        let mut replaced = false;
        for a in articles.iter_mut() {
            if a.id == article.id {
                *a = article.clone();
                replaced = true;
                break;
            }
        }
        if !replaced {
            articles.push(article);
        }
        self.write_all(feed, &articles)
    }
}

pub struct MockStorage {}

impl MockStorage {
    pub fn new() -> Self {
        MockStorage {}
    }
}

impl FeedStorage for MockStorage {
    fn get_all_articles(&self, _feed: &str) -> Result<Vec<Article>, StorageError> {
        Ok(vec![])
    }

    fn get_latest_article(&self, _feed: &str) -> Result<Article, StorageError> {
        Err(StorageError::FeedEmpty)
    }

    fn store_article(&mut self, _feed: &str, _article: Article) -> Result<(), StorageError> {
        Ok(())
    }
}
