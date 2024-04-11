use anyhow::Result;
use genanki_rs::{Deck, Field, Model, Note, Template};

use crate::{
    db::Word,
    info::TrInfo,
    query::{app_anki_values, list_anki_fields},
};

const CSS: &str = ".card {
  font-family: arial;
  font-size: 20px;
  text-align: center;
  color: black;
  background-color: white;
}";

const AFMT: &str = r#"{{FrontSide}}
<hr id="answer">
{{Reading}}<br>
{{Translate}}<br>
{{Transcription}}<br>
{{Picture}}"#;

pub struct DeckWriter {
    model: Model,
    deck: Deck,
    info: TrInfo,
}

impl DeckWriter {
    pub fn new(info: TrInfo) -> Self {
        let model = Model::new(
            10964854234534,
            "Reword",
            list_anki_fields()
                .into_iter()
                .map(|f| Field::new(f.as_str()))
                .collect(),
            vec![Template::new(&format!(
                "{} - {}",
                info.learn_lang.dislpay(),
                info.tr_lang.dislpay()
            ))
            .qfmt("{{Word}}")
            .afmt(AFMT)],
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
    pub fn word(&mut self, w: Word) -> Result<()> {
        self.deck.add_note(Note::new_with_options(
            self.model.clone(),
            app_anki_values(self.info.app, &w)
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

/*#[derive(Debug)]
struct AnkiFields {
    pub word: Option<String>,
    pub reading: Option<String>,
    pub transcription: String,
    pub translate: Option<String>,
    pub picture: Option<String>,
}*/
