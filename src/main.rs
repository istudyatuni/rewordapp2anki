use std::{
    collections::HashSet,
    io::{Cursor, Read},
    path::{Path, PathBuf},
    time::Instant,
};

use anyhow::{anyhow, Result};
use clap::Parser;
use inquire::{Confirm, MultiSelect, Select, Text};
use zip::ZipArchive;

use crate::{
    db::{Category, DB},
    deck::DeckWriter,
    info::{App, Language, TrInfo},
    inquire_autocomplete_path::FilePathCompleter,
    query::{app_apk_db_path, app_languages},
};

mod args;
mod db;
mod deck;
mod info;
mod inquire_autocomplete_path;
mod query;

/// Minimum number of words to measure approximated export time
const APPROX_BOUND: usize = 100;

const DEFAULT_OUTPUT_FILE: &str = "reword.apkg";

#[derive(Debug)]
struct Input {
    tr: TrInfo,
    db_path: PathBuf,
    output_path: String,
}

fn main() -> Result<()> {
    let args = args::Cli::parse();
    let input = ask(args.no_cache)?;
    let db = DB::new(input.db_path)?;

    let total_words = db.words_count()?;
    let words = db.list_words(input.tr.clone())?;
    if total_words > words.len() {
        println!(
            "Not all words are available for {} language, total words in database: {}",
            input.tr.tr_lang.display(),
            total_words
        )
    }

    // select categories
    let categories = db.list_categories(input.tr.tr_lang)?;
    let words: Vec<_> = if let Some(categories) = ask_categories(categories)? {
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

    // export with timer
    let timer = Instant::now();
    let mut deck = DeckWriter::new(input.tr);
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

/// Ask for:
///
/// - App
/// - Path to APK file (if db for this app is not cached)
/// - Translate language
/// - Where to save exported collection
///
/// If path to APK is given, extract and cache db
fn ask(no_cache: bool) -> Result<Input> {
    let app: App = Select::new("App to import:", App::SUPPORTED.to_vec()).prompt()?;

    let current_dir = std::env::current_dir().unwrap();
    let help_message = format!("Current directory: {}", current_dir.display());

    let db_path = db_cache_path(app)?;
    if !db_path.exists() || no_cache {
        let apk_path = Text::new("Path to APK file:")
            .with_autocomplete(FilePathCompleter::default())
            .with_help_message(&help_message)
            .prompt()?;
        extract_db(app, apk_path, &db_path)?;
    }

    let learn_lang = app.into();
    let tr_lang: Language =
        Select::new("Translate language:", app_languages(app).to_vec()).prompt()?;

    let output_path = Text::new("Path to exported .apkg collection:")
        .with_autocomplete(FilePathCompleter::default())
        .with_help_message(&help_message)
        .with_initial_value(DEFAULT_OUTPUT_FILE)
        .prompt()?;

    Ok(Input {
        tr: TrInfo {
            app,
            learn_lang,
            tr_lang,
        },
        db_path,
        output_path,
    })
}

fn ask_categories(categories: Vec<Category>) -> Result<Option<Vec<Category>>> {
    if !Confirm::new("Select specific categories?")
        .with_default(false)
        .prompt()?
    {
        return Ok(None);
    }

    let total_categories = categories.len();
    let result = MultiSelect::new("Select categories:", categories)
        .with_all_selected_by_default()
        .prompt()?;
    if total_categories == result.len() {
        Ok(None)
    } else {
        Ok(Some(result))
    }
}

fn db_cache_path(app: App) -> Result<PathBuf> {
    let db_path = dirs::cache_dir()
        .ok_or_else(|| anyhow!("cannot determine cache directory"))?
        .join(env!("CARGO_PKG_NAME"))
        .join("db");
    std::fs::create_dir_all(&db_path)?;
    let db_path = db_path.join(format!("{}.db", app.kind()));
    Ok(db_path)
}

fn extract_db(app: App, apk_path: impl AsRef<Path>, db_path: impl AsRef<Path>) -> Result<()> {
    let apk = std::fs::read(apk_path)?;
    let mut zip = ZipArchive::new(Cursor::new(apk))?;
    let mut db = zip.by_name(app_apk_db_path(app))?;

    let mut buf = Vec::with_capacity(db.size() as usize);
    db.read_to_end(&mut buf)?;
    std::fs::write(&db_path, buf)?;

    Ok(())
}
