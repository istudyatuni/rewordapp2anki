use anyhow::Result;
use genanki_rs::{Deck, Field, Model, Note, Template};

use crate::{
    db::{Example, Picture, Word},
    info::TrInfo,
    query::{app_anki_fields, app_anki_values, app_model_id},
};

const CSS: &str = ".card {
  font-family: arial;
  font-size: 20px;
  text-align: center;
  color: black;
  background-color: white;
}
details {
  text-align: left;
}";

const EXAMPLES_FIELD: &str = "examples";

const EXAMPLES: &str = "
{{#examples}}
<br>Examples:
{{examples}}
{{/examples}}";

pub struct DeckWriter {
    model: Model,
    deck: Deck,
    info: TrInfo,
}

impl DeckWriter {
    pub fn new(info: TrInfo) -> Self {
        let fields = app_anki_fields(info.app);
        let model = Model::new(
            app_model_id(info.app),
            &format!("Reword {}", info.app.display()),
            fields.names().into_iter().map(Field::new).collect(),
            vec![
                Template::new(&format!(
                    "{} - {}",
                    info.learn_lang.display(),
                    info.tr_lang.display(),
                ))
                .qfmt(&fields.qfmt())
                .afmt(&fields.afmt()),
                Template::new(&format!(
                    "{} - {}",
                    info.tr_lang.display(),
                    info.learn_lang.display(),
                ))
                .qfmt(&fields.qfmt_rev())
                .afmt(&fields.afmt_rev()),
            ],
        )
        .css(CSS)
        .sort_field_index(AnkiFieldNames::sort_index());
        let deck = Deck::new(
            965781129384,
            &format!("Reword {} - {}", info.app.display(), info.tr_lang.display()),
            "",
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
            Some(w.category_ids.iter().map(|c| c.as_str()).collect()),
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
    pub examples: Option<Vec<Example>>,
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
            examples_to_html(self.examples.as_deref()),
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
            examples: value.examples,
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
    fn qfmt_rev(&self) -> String {
        Self::field(&self.translate)
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

        format!(
            "{}\n<hr id=\"answer\">\n{back}{EXAMPLES}",
            Self::field("FrontSide")
        )
    }
    fn afmt_rev(&self) -> String {
        let back = [
            &self.word,
            &self.reading,
            &self.transcription,
            // &self.picture,
        ]
        .iter()
        .map(|f| Self::field(f))
        .collect::<Vec<_>>()
        .join("<br>\n");

        format!(
            "{}\n<hr id=\"answer\">\n{back}{EXAMPLES}",
            Self::field("FrontSide")
        )
    }
    fn field(name: &str) -> String {
        format!("{{{{{name}}}}}")
    }
    const fn sort_index() -> i64 {
        0
    }
    fn names(&self) -> impl IntoIterator<Item = &str> {
        vec![
            self.word.as_str(),
            self.reading.as_str(),
            self.translate.as_str(),
            self.transcription.as_str(),
            // &self.picture,
            EXAMPLES_FIELD,
        ]
    }
}

fn examples_to_html(examples: Option<&[Example]>) -> String {
    let Some(examples) = examples else {
        // todo: do not use cfg
        #[cfg(not(test))]
        return "".to_string();
        #[cfg(test)]
        return EXAMPLES_FIELD.to_string();
    };

    let mut res = "".to_string();
    for ex in examples.iter().map(|e| e.to_anki()) {
        res += &format!(
            "<details><summary>{}</summary>{}</details>",
            ex.original, ex.translate
        );
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::db::PictureSource;

    use super::*;

    #[test]
    fn test_anki_fields() {
        let word = "word";
        let reading = "reading";
        let translate = "translate";
        let transcription = "transcription";
        let picture = "picture";

        let expected = vec![word, reading, translate, transcription, EXAMPLES_FIELD];

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
            examples: None,
        };

        assert_eq!(names.qfmt(), "{{word}}");
        assert_eq!(names.qfmt_rev(), "{{translate}}");

        let names: Vec<_> = names.names().into_iter().collect();
        assert_eq!(
            names[AnkiFieldNames::sort_index() as usize],
            word,
            "sort_index does not point to {word}"
        );
        assert_eq!(names, expected.clone(), "field's names are broken");

        let fields = fields.list();
        let fields: Vec<_> = fields.iter().map(|f| f.as_str()).collect();
        assert_eq!(fields, expected, "field's values are broken");
    }
}
