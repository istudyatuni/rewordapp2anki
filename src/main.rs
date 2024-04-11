#![allow(unused)]

use std::time::Instant;

use anyhow::Result;

use crate::{
    db::DB,
    deck::DeckWriter,
    info::{Language, TrInfo},
};

mod db;
mod deck;
mod info;
mod query;

const DB_FILE: &str = "data/reword-app-jap/res/raw/englishwordsdb";
const OUTPUT_FILE: &str = "target/reword.apkg";

fn main() -> Result<()> {
    let mut db = DB::new(DB_FILE)?;

    // let categories = db.list_categories()?;
    let words = db.list_words(info::App::Japanese)?;
    println!("words count: {}", words.len());

    // let categories: HashMap<String, &db::Category> = HashMap::from_iter(categories.iter().map(|c| (c.id.clone(), c)));

    /*let timer = Instant::now();
    let mut deck = DeckWriter::new(TrInfo {
        app: info::App::Japanese,
        learn_lang: Language::Japanese,
        tr_lang: Language::Russian,
    });
    for w in words {
        deck.word(w)?;
    }
    println!("collected in {:?}", timer.elapsed());
    deck.export(OUTPUT_FILE)?;*/

    Ok(())
}
