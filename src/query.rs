pub use func::*;

mod func {
    use rusqlite::Row;

    use crate::{
        db::{Picture, Word},
        deck::{AnkiFieldNames, AnkiFields},
        info::{App, Language, TrInfo},
    };

    use super::*;

    pub fn app_apk_db_path(app: App) -> &'static str {
        match app {
            App::Russian => "res/kv",
            _ => "res/raw/englishwordsdb",
        }
    }

    pub fn app_sql(info: TrInfo) -> String {
        match info.app {
            App::English => eng::words(info),
            App::Japanese => jap::words(info),
            App::Russian => rus::words(info),
            _ => todo!("app not yet supported"),
        }
    }

    pub fn app_query_map(_app: App) -> impl FnMut(&Row<'_>) -> rusqlite::Result<Word> {
        map_row
    }

    fn map_row(r: &Row<'_>) -> rusqlite::Result<Word> {
        Ok(Word {
            id: r.get("id")?,
            word: r.get("word")?,
            transcription: r.get("transcription")?,
            picture: Picture::new(r.get("picture_source")?, r.get("picture_source_id")?),
            reading: r.get("reading")?,
            translate: r.get("translate")?,
            category_ids: vec![r.get("category_id")?],
        })
    }

    /*pub fn app_sql_params(info: TrInfo) -> Vec<String> {
        match info.app {
            App::English => eng::params(info),
            App::Japanese => jap::params(info),
            _ => todo!("app not yet supported"),
        }
    }*/

    pub fn app_anki_fields(app: App) -> AnkiFieldNames {
        match app {
            App::Japanese => jap::anki_fields(),
            _ => AnkiFieldNames::default(),
        }
    }

    pub fn app_anki_values(app: App, w: &Word) -> AnkiFields {
        match app {
            App::Japanese => jap::anki_values(w),
            _ => w.clone().into(),
        }
    }

    pub fn app_languages(app: App) -> &'static [Language] {
        match app {
            App::English => &eng::LANGUAGES,
            App::Japanese => &jap::LANGUAGES,
            App::Russian => &rus::LANGUAGES,
            _ => todo!("app not yet supported"),
        }
    }
}

mod eng {
    use crate::info::{Language, TrInfo};

    const WORDS: &str = "select
           w.id,
           w.word,
           w.transcription,
           null as reading,
           {LANG} as translate,
           wc.category_id,
           p.source as picture_source,
           p.source_id as picture_source_id
         from word w
         join word_category wc
           on w.id = wc.word_id
         full outer join picture p
           on p.id = w.picture_id";

    pub fn words(info: TrInfo) -> String {
        WORDS.replace("{LANG}", &format!("w.{}", info.tr_lang.kind()))
    }

    pub const LANGUAGES: [Language; 10] = [
        Language::Chinese,
        Language::Dutch,
        Language::French,
        Language::Deutsch,
        Language::Italian,
        Language::Japanese,
        Language::Korean,
        Language::Russian,
        Language::Spanish,
        Language::Turkish,
    ];
}

mod jap {
    use crate::{
        db::Word,
        deck::{AnkiFieldNames, AnkiFields},
        info::{Language, TrInfo},
    };

    const WORDS: &str = "select
           w.id,
           w.kanji as word,
           w.word as reading,
           w.transcription,
           {LANG} as translate,
           wc.category_id,
           p.source as picture_source,
           p.source_id as picture_source_id
         from word w
         join word_category wc
           on w.id = wc.word_id
         full outer join picture p
           on p.id = w.picture_id";

    pub fn words(info: TrInfo) -> String {
        WORDS.replace("{LANG}", &format!("w.{}", info.tr_lang.kind()))
    }

    pub const LANGUAGES: [Language; 2] = [Language::English, Language::Russian];

    pub fn anki_fields() -> AnkiFieldNames {
        AnkiFieldNames {
            word: "Kanji".to_string(),
            reading: "Kana".to_string(),
            transcription: "Romaji".to_string(),
            ..Default::default()
        }
    }

    pub fn anki_values(w: &Word) -> AnkiFields {
        AnkiFields {
            word: w.word.clone().or(w.reading.clone()),
            reading: w.word.clone().and(w.reading.clone()),
            ..w.clone().into()
        }
    }
}

mod rus {
    use crate::info::{Language, TrInfo};

    const WORDS: &str = "select
           w.id,
           w.word,
           null as reading,
           w.transcription,
           {LANG} as translate,
           wc.category_id,
           p.source as picture_source,
           p.source_id as picture_source_id
         from word w
         join word_category wc
           on w.id = wc.word_id
         full outer join picture p
           on p.id = w.picture_id";

    pub fn words(info: TrInfo) -> String {
        WORDS.replace("{LANG}", &format!("w.{}", info.tr_lang.kind()))
    }

    pub const LANGUAGES: [Language; 3] = [Language::Deutsch, Language::English, Language::French];
}
