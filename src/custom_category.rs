use std::{collections::HashMap, path::Path};

use anyhow::Result;
use serde::Deserialize;

use crate::{info::Language, zip::extract_by_name};

pub fn read_from_file(path: impl AsRef<Path>) -> Result<Category> {
    let data = std::fs::read(path)?;
    let data = extract_by_name(data, "data")?;
    let data = String::from_utf8(data)?;
    let data: CategoryRaw = serde_json::from_str(&data)?;

    let mut words = vec![];
    let mut detected_lang = None;
    for w in data.words {
        if w.tr.len() != 2 {
            eprintln!("[warn] more than 2 translation keys in word: {:?}", w.tr);
        }
        let mut tr = None;
        let mut examples: Option<Vec<Example>> = None;
        let mut lang = None;
        for (key, value) in w.tr {
            if key.starts_with("exm_") {
                examples = Some(serde_json::from_str(&value)?);
            } else {
                tr = Some(value);
                lang = Some(key);
            }
        }
        if tr.is_none() && examples.is_none() {
            eprintln!("[error] no translation keys in word {:?}, skipping", w.word);
            break;
        }

        let translation = tr.unwrap();
        let examples = examples.unwrap();
        let lang = lang.unwrap();
        if detected_lang.is_none() {
            detected_lang = Some(Language::from_db_name(&lang));
        }

        words.push(Word {
            word: w.word,
            transcription: w.transcription,
            translation,
            examples,
        });
    }

    Ok(Category {
        ty: data.ty,
        flavor: data.flavor,
        words,
        title: detected_lang.and_then(|lang| data.titles_tr.get(&lang.db_name()).cloned()),
    })
}

#[derive(Debug)]
pub struct Category {
    #[expect(unused)]
    pub ty: String,
    pub flavor: String,
    pub words: Vec<Word>,

    pub title: Option<String>,
}

#[derive(Debug)]
pub struct Word {
    pub word: String,
    pub translation: String,
    pub transcription: Option<String>,
    pub examples: Vec<Example>,
}

#[derive(Debug, Deserialize)]
pub struct Example {
    #[serde(rename = "o")]
    pub original: String,
    #[serde(rename = "t")]
    pub translation: String,
}

#[derive(Debug, Deserialize)]
#[expect(unused)]
struct CategoryRaw {
    #[serde(rename = "type")]
    ty: String,
    flavor: String,
    icon: String,
    version: String,
    words: Vec<WordRaw>,

    #[serde(flatten)]
    titles_tr: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
struct WordRaw {
    #[serde(rename = "wrd")]
    word: String,
    #[serde(rename = "tsc")]
    transcription: Option<String>,

    // unused fields
    #[expect(unused, reason = "ignore but do not fail parsing")]
    #[serde(default)]
    pic_cst: bool,
    #[expect(unused)]
    #[serde(default)]
    pic_src: String,
    #[expect(unused)]
    #[serde(default)]
    pic_id: String,

    #[serde(flatten)]
    tr: HashMap<String, String>,
}
