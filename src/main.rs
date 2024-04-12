use std::{
    io::{Cursor, Read},
    path::{Path, PathBuf},
    time::Instant,
};

use anyhow::{anyhow, Result};
use clap::Parser;
use info::App;
use inquire::{Select, Text};
use inquire_autocomplete_path::FilePathCompleter;
use query::{app_apk_db_path, app_languages};
use zip::ZipArchive;

use crate::{
    db::DB,
    deck::DeckWriter,
    info::{Language, TrInfo},
};

mod args;
mod db;
mod deck;
mod info;
mod inquire_autocomplete_path;
mod query;

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

    let words = db.list_words(input.tr.clone())?;
    println!("Words count: {}", words.len());

    // let categories = db.list_categories()?;
    // let categories: HashMap<String, &db::Category> = HashMap::from_iter(categories.iter().map(|c| (c.id.clone(), c)));

    let timer = Instant::now();
    let mut deck = DeckWriter::new(input.tr);
    if words.len() > APPROX_BOUND {
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
