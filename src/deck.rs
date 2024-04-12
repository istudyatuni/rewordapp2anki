use anyhow::Result;
use genanki_rs::{Deck, Field, Model, Note, Template};

use crate::{
    db::{Picture, Word},
    info::TrInfo,
    query::{app_anki_fields, app_anki_values},
};

const CSS: &str = ".card {
  font-family: arial;
  font-size: 20px;
  text-align: center;
  color: black;
  background-color: white;
}";

pub struct DeckWriter {
    model: Model,
    deck: Deck,
    info: TrInfo,
}

impl DeckWriter {
    pub fn new(info: TrInfo) -> Self {
        let fields = app_anki_fields(info.app);
        let model = Model::new(
            10964854234534,
            "Reword",
            fields.names().into_iter().map(|f| Field::new(f)).collect(),
            vec![Template::new(&format!(
                "{} - {}",
                info.learn_lang.display(),
                info.tr_lang.display()
            ))
            .qfmt(&fields.qfmt())
            .afmt(&fields.afmt())],
        )
        .css(CSS)
        .sort_field_index(0);
        let deck = Deck::new(
            965781129384,
            &format!("Reword {}", info.app.display()),
            "test deck",
        );
        Self { model, deck, info }
    }
    pub fn word(&mut self, w: &Word) -> Result<()> {
        self.deck.add_note(Note::new_with_options(
            self.model.clone(),
            app_anki_values(self.info.app, w)
                .list()
                .iter()
                .map(|v| v.as_str())
                .collect(),
            None,
            Some(w.category_id.iter().map(|c| c.as_str()).collect()),
            Some(&format!("reword-{}-{}", self.info.app.kind(), w.id)),
        )?);
        Ok(())
    }
    pub fn export(self, path: &str) -> Result<()> {
        self.deck.write_to_file(path)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct AnkiFields {
    pub word: Option<String>,
    pub reading: Option<String>,
    pub transcription: String,
    pub translate: Option<String>,
    pub picture: Option<Picture>,
}

impl AnkiFields {
    fn list(&self) -> Vec<String> {
        vec![
            self.word.clone().unwrap_or_default(),
            self.reading.clone().unwrap_or_default(),
            self.translate.clone().unwrap_or_default(),
            self.transcription.clone(),
            /*self.picture
            .clone()
            .map(|p| format!("{}:{}", p.source, p.source_id))
            .unwrap_or_default(),*/
        ]
    }
}

impl From<Word> for AnkiFields {
    fn from(value: Word) -> Self {
        Self {
            word: value.word,
            reading: value.reading,
            transcription: value.transcription,
            translate: value.translate,
            picture: value.picture,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AnkiFieldNames {
    pub word: String,
    pub reading: String,
    pub transcription: String,
    pub translate: String,
    pub picture: String,
}

impl Default for AnkiFieldNames {
    fn default() -> Self {
        Self {
            word: "Word".to_string(),
            reading: "Reading".to_string(),
            transcription: "Transcription".to_string(),
            translate: "Translate".to_string(),
            picture: "Picture".to_string(),
        }
    }
}

impl AnkiFieldNames {
    fn qfmt(&self) -> String {
        Self::field(&self.word)
    }
    fn afmt(&self) -> String {
        let back = [
            &self.reading,
            &self.translate,
            &self.transcription,
            // &self.picture,
        ]
        .iter()
        .map(|f| Self::field(f))
        .collect::<Vec<_>>()
        .join("<br>\n");

        format!("{}\n<hr id=\"answer\">\n{back}", Self::field("FrontSide"))
    }
    fn field(name: &str) -> String {
        format!("{{{{{name}}}}}")
    }
    fn names(&self) -> impl IntoIterator<Item = &String> {
        vec![
            &self.word,
            &self.reading,
            &self.translate,
            &self.transcription,
            // &self.picture,
        ]
    }
}

#[cfg(test)]
mod tests {
    use crate::db::PictureSource;

    use super::*;

    /// Test for matching of fields order
    #[test]
    fn test_anki_fields() {
        let word = "word";
        let reading = "reading";
        let translate = "translate";
        let transcription = "transcription";
        let picture = "picture";

        let expected = vec![word, reading, translate, transcription];

        let names = AnkiFieldNames {
            word: word.to_string(),
            reading: reading.to_string(),
            transcription: transcription.to_string(),
            translate: translate.to_string(),
            picture: picture.to_string(),
        };
        let fields = AnkiFields {
            word: Some(word.to_string()),
            reading: Some(reading.to_string()),
            transcription: transcription.to_string(),
            translate: Some(translate.to_string()),
            picture: Some(Picture {
                source: PictureSource::Pixabay,
                source_id: "asdf".to_string(),
            }),
        };

        let names: Vec<_> = names.names().into_iter().collect();
        assert_eq!(names, expected.clone());

        let fields = fields.list();
        let fields: Vec<_> = fields.iter().map(|f| f.as_str()).collect();
        assert_eq!(fields, expected);
    }
}
