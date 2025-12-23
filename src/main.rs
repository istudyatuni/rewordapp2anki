use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    time::Instant,
};

use anyhow::Result;
use clap::Parser;

use crate::{
    ask::{SourceFile, UserInput},
    db::{DB, Example, Word},
    deck::DeckWriter,
    info::AppTranslationInfo,
};

mod args;
mod ask;
mod custom_category;
mod db;
mod deck;
mod info;
mod inquire_autocomplete_path;
mod query;
mod zip;

/// Minimum number of words to measure approximated export time
const APPROX_BOUND: usize = 100;

const DEFAULT_OUTPUT_FILE: &str = "reword.apkg";

fn main() -> Result<()> {
    let args = args::Cli::parse();
    let input = ask::ask(args.no_cache)?;
    let words = match &input.source_path {
        SourceFile::DB(path) => import_words(&input.tr, path)?,
        SourceFile::CustomCategories(paths) => import_custom_categories_words(&input.tr, paths)?,
    };
    export_deck(input, words)?;
    Ok(())
}

fn import_words(tr: &AppTranslationInfo, db_path: impl AsRef<Path>) -> Result<Vec<Word>> {
    let db = DB::new(db_path)?;

    let total_words = db.words_count()?;
    let words = db.list_words(tr)?;
    if total_words > words.len() {
        println!(
            "Not all words are available for {} language, total words in database: {}",
            tr.tr_lang.display(),
            total_words
        )
    }

    // select categories
    let categories = db.list_categories(tr.tr_lang)?;
    let words: Vec<_> = if let Some(categories) = ask::ask_categories(categories)? {
        println!("All words count: {}", words.len());

        let categories: HashSet<_> = categories.into_iter().map(|c| c.id).collect();
        words
            .into_iter()
            .filter(|w| w.category_ids.iter().any(|c| categories.contains(c)))
            .collect()
    } else {
        words
    };
    println!("Words to export: {}", words.len());

    Ok(words)
}

fn import_custom_categories_words(
    tr: &AppTranslationInfo,
    custom_categories: &[PathBuf],
) -> Result<Vec<Word>> {
    let mut words = vec![];
    let mut id = 1;
    for p in custom_categories {
        eprintln!("[info] handling {}", p.display());
        let category = custom_category::read_from_file(p)?;

        if category.flavor != tr.app.kind() {
            eprintln!(
                "[warn] category \"{}\" is from different app",
                category.title.as_deref().unwrap_or_default()
            );
        }

        for w in category.words {
            words.push(Word {
                id,
                word: Some(w.word),
                transcription: w.transcription,
                picture: None,
                reading: None,
                translate: Some(w.translation),
                examples: Some(
                    w.examples
                        .into_iter()
                        .map(|e| Example {
                            original: e.original,
                            translate: e.translation,
                        })
                        .collect(),
                ),
                category_ids: category
                    .title
                    .iter()
                    .map(|t| t.to_lowercase().replace(" ", "-"))
                    .collect(),
            });
            id += 1;
        }
    }

    Ok(words)
}

fn export_deck(input: UserInput, words: Vec<db::Word>) -> Result<(), anyhow::Error> {
    let timer = Instant::now();
    let mut deck = DeckWriter::new(input.tr, input.mark_custom_on_export);
    if words.len() > APPROX_BOUND / 2 {
        let timer = Instant::now();
        for w in &words[..APPROX_BOUND] {
            deck.word(w)?;
        }
        let t = timer.elapsed() / APPROX_BOUND as u32 * words.len() as u32;
        println!("Approximated export time: {t:?}");

        for w in &words[APPROX_BOUND..] {
            deck.word(w)?;
        }
    } else {
        for w in &words {
            deck.word(w)?;
        }
    }
    println!("Exported in {:?}, saving collection", timer.elapsed());
    deck.export(&input.output_path)?;
    println!("File saved in {}", input.output_path);

    Ok(())
}
