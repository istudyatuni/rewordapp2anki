use std::{collections::HashMap, fmt::Display, path::Path};

use anyhow::{anyhow, Result};
use rusqlite::Connection;

use crate::{info::App, query::app_sql};

#[derive(Debug)]
pub struct DB {
    conn: Connection,
}

impl DB {
    pub fn new(file: impl AsRef<Path>) -> Result<Self> {
        Ok(Self {
            conn: Connection::open(file)?,
        })
    }
    /*pub fn list_categories(&self) -> Result<Vec<Category>> {
        let mut st = self.conn.prepare(Category::LIST_SQL)?;
        let categories: Vec<_> = st
            .query_map([], |r| {
                Ok(Category {
                    id: r.get(0)?,
                    name_eng: r.get(1)?,
                    name_rus: r.get(2)?,
                })
            })?
            .filter_map(|c| c.ok())
            .collect();
        Ok(categories)
    }*/
    pub fn list_words(&self, app: App) -> Result<Vec<Word>> {
        let mut st = self.conn.prepare(app_sql(app))?;
        let words = st
            .query_map([], |r| {
                Ok(Word {
                    id: r.get("id")?,
                    word: r.get("word")?,
                    transcription: r.get("transcription")?,
                    picture: Picture::new(r.get("picture_source")?, r.get("picture_source_id")?),
                    reading: r.get("reading")?,
                    translate: r.get("translate")?,
                    category_id: vec![r.get("category_id")?],
                })
            })?
            .filter_map(|c| c.ok())
            .collect::<Vec<_>>();
        fold_categories(words)
    }
    /*pub fn delete_words(&mut self, ids: &[i64]) -> Result<()> {
        let tx = self.conn.transaction()?;
        for &i in ids {
            tx.execute("delete from word where id = ?", [i])?;
        }
        tx.commit()?;
        Ok(())
    }*/
}

/*#[derive(Debug)]
pub struct Category {
    pub id: String,
    pub name_eng: String,
    pub name_rus: String,
}

impl Category {
    const LIST_SQL: &'static str =
        "select id, name_eng, name_rus from category where is_custom = 0";
}*/

#[derive(Debug, Clone)]
pub struct Word {
    pub id: i64,
    pub word: Option<String>,
    pub transcription: String,
    pub picture: Option<Picture>,
    pub reading: Option<String>,
    pub translate: Option<String>,
    pub category_id: Vec<String>,
}

/// Merge multiple equal words with categories to one word
fn fold_categories(words: Vec<Word>) -> Result<Vec<Word>> {
    let mut map: HashMap<i64, Word> = HashMap::with_capacity(words.len());
    for w in words {
        if let Some(e) = map.get_mut(&w.id) {
            e.category_id.extend(w.category_id);
        } else {
            map.insert(w.id, w);
        }
    }
    Ok(map.into_values().collect())
}

#[derive(Debug, Clone)]
pub struct Picture {
    pub source: PictureSource,
    pub source_id: String,
}

impl Picture {
    pub fn new(source: Option<String>, source_id: Option<String>) -> Option<Self> {
        Some(Self {
            source: source?.as_str().into(),
            source_id: source_id?,
        })
    }
}

#[derive(Debug, Clone)]
pub enum PictureSource {
    Pixabay,
    Pexels,
    Other(String),
}

impl From<&str> for PictureSource {
    fn from(value: &str) -> Self {
        match value {
            "pixabay" => Self::Pixabay,
            "pexels" => Self::Pexels,
            v => Self::Other(v.to_string()),
        }
    }
}

impl Display for PictureSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            PictureSource::Pixabay => "pixabay",
            PictureSource::Pexels => "pexels",
            PictureSource::Other(s) => s.as_str(),
        };
        f.pad(s)
    }
}
