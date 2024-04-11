//! SQL should return fields
//! - `id`
//! - `word`
//! - `reading`
//! - `transcription`
//! - `translate`
//! - `category_id`
//! - `picture_source`
//! - `picture_source_id`

use rusqlite::Row;

use crate::{
    db::{Picture, Word},
    info::App,
};

mod jap {
    use crate::db::{Picture, Word};

    // todo: do not ignore translation language
    pub const WORDS: &str = "select
           w.id,
           w.kanji as word,
           w.word as reading,
           w.transcription,
           w.rus as translate,
           wc.category_id,
           p.source as picture_source,
           p.source_id as picture_source_id
         from word w
         join word_category wc
           on w.id = wc.word_id
         full outer join picture p
           on p.id = w.picture_id";

    pub fn anki_values(w: &Word) -> Vec<String> {
        let (word, reading) = if let Some(word) = &w.word {
            (word.clone(), w.reading.clone().unwrap_or_default())
        } else {
            (w.reading.clone().unwrap_or_default(), "".to_string())
        };
        vec![
            word,
            reading,
            w.transcription.clone(),
            w.translate.clone().unwrap_or_default(),
            w.picture
                .clone()
                .map(|p| format!("{}:{}", p.source, p.source_id))
                .unwrap_or_default(),
        ]
    }
}

pub fn app_sql(app: App) -> &'static str {
    match app {
        App::Japanese => jap::WORDS,
        _ => unreachable!(),
    }
}

pub fn app_query_map(app: App) -> impl FnMut(&Row<'_>) -> rusqlite::Result<Word> {
    /*match app {
        // App::Japanese => jap::map_row,
        _ => map_row,
    }*/
    map_row
}

pub fn list_anki_fields() -> Vec<String> {
    ["Word", "Reading", "Transcription", "Translate", "Picture"]
        .into_iter()
        .map(|f| f.to_owned())
        .collect()
}

pub fn app_anki_values(app: App, w: &Word) -> Vec<String> {
    match app {
        App::Japanese => jap::anki_values(w),
        _ => anki_values(w),
    }
}

fn anki_values(w: &Word) -> Vec<String> {
    vec![
        w.word.clone().unwrap_or_default(),
        w.reading.clone().unwrap_or_default(),
        w.transcription.clone(),
        w.translate.clone().unwrap_or_default(),
        w.picture
            .clone()
            .map(|p| format!("{}:{}", p.source, p.source_id))
            .unwrap_or_default(),
    ]
}

fn map_row(r: &Row<'_>) -> rusqlite::Result<Word> {
    Ok(Word {
        id: r.get("id")?,
        word: r.get("word")?,
        transcription: r.get("transcription")?,
        picture: Picture::new(r.get("picture_source")?, r.get("picture_source_id")?),
        reading: r.get("reading")?,
        translate: r.get("translate")?,
        category_id: vec![r.get("category_id")?],
    })
}
