use std::{collections::HashMap, fmt::Display, path::Path};

use anyhow::Result;
use rusqlite::Connection;
use serde::Deserialize;

use crate::{
    info::{Language, TrInfo},
    query::{app_query_map, app_sql},
};

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
    pub fn list_categories(&self, lang: Language) -> Result<Vec<Category>> {
        let mut st = self.conn.prepare(&Category::list_sql(lang))?;
        let categories: Vec<_> = st
            .query_map([], |r| {
                Ok(Category {
                    id: r.get("id")?,
                    name: r.get("name")?,
                    words_count: r.get("words_count")?,
                })
            })?
            .filter_map(|c| {
                c.inspect_err(|e| eprintln!("failed to map category: {e}"))
                    .ok()
            })
            .collect();
        Ok(categories)
    }
    pub fn words_count(&self) -> Result<usize> {
        let mut st = self.conn.prepare("select count(*) as count from word")?;
        Ok(st.query_row([], |r| r.get("count"))?)
    }
    pub fn list_words(&self, info: TrInfo) -> Result<Vec<Word>> {
        let mut st = self.conn.prepare(&app_sql(info.clone()))?;
        let words = st
            .query_map([], app_query_map(info.app))?
            .filter_map(|c| c.inspect_err(|e| eprintln!("failed to map word: {e}")).ok())
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

#[derive(Debug)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub words_count: i64,
}

impl Category {
    const LIST_SQL: &'static str = "select
           c.id,
           {LANG} as name,
           (
             select
               count(*)
               from word_category wc
               where wc.category_id = c.id
           ) as words_count
         from category c
         where c.is_custom = 0";

    fn list_sql(lang: Language) -> String {
        Self::LIST_SQL.replace("{LANG}", &format!("c.name_{}", lang.kind()))
    }
}

impl Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({} words)", self.name, self.words_count)
    }
}

#[derive(Debug, Clone)]
pub struct Word {
    pub id: i64,
    pub word: Option<String>,
    pub transcription: String,
    pub picture: Option<Picture>,
    pub reading: Option<String>,
    pub translate: Option<String>,
    pub examples: Option<Vec<Example>>,
    pub category_ids: Vec<String>,
}

/// Merge multiple equal words with categories to one word
fn fold_categories(words: Vec<Word>) -> Result<Vec<Word>> {
    let mut map: HashMap<i64, Word> = HashMap::with_capacity(words.len());
    for w in words {
        if let Some(e) = map.get_mut(&w.id) {
            e.category_ids.extend(w.category_ids);
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

#[derive(Debug, Clone, Deserialize)]
pub struct Example {
    #[serde(rename(deserialize = "o"))]
    pub original: String,
    #[serde(rename(deserialize = "t"))]
    pub translate: String,
}

impl Example {
    pub fn from_db(s: Option<String>) -> Option<Vec<Self>> {
        let s = s?;
        serde_json::from_str(&s)
            .inspect_err(|e| eprintln!("failed to decode examples {s}\nerror: {e}"))
            .ok()
    }
    pub fn to_anki(&self) -> Self {
        Self {
            original: hash2bold(&self.original),
            translate: hash2bold(&self.translate),
        }
    }
}

/// "asdf #asdf# asdf" -> "asdf <b>asdf</b> asdf"
fn hash2bold(s: &str) -> String {
    const HASH: char = '#';
    s.split_inclusive(HASH)
        .fold((true, "".to_string()), |(is_open, acc), s| {
            let sep = if is_open { "<b>" } else { "</b>" };
            (!is_open, acc + &s.replace(HASH, sep))
        })
        .1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash2bold() {
        let table = [
            ("asdf", "asdf"),
            ("#asdf# asdf", "<b>asdf</b> asdf"),
            ("asdf#asdf#asdf", "asdf<b>asdf</b>asdf"),
            ("asdf #asdf# asdf", "asdf <b>asdf</b> asdf"),
            // invalid states
            ("asdf#asdf#asdf#", "asdf<b>asdf</b>asdf<b>"),
            ("asdf#asdf#asdf#asdf", "asdf<b>asdf</b>asdf<b>asdf"),
        ];
        for (input, expected) in table {
            assert_eq!(hash2bold(input), expected);
        }
    }
}
