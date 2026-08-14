use std::{collections::HashSet, fmt::Display, path::PathBuf};

use anyhow::{Result, anyhow};
use inquire::{Confirm, MultiSelect, Select, Text, ui::RenderConfig};
use inquire_derive::Selectable;

use crate::{
    DEFAULT_OUTPUT_FILE,
    db::Category,
    info::{App, AppTranslationInfo, Language},
    inquire_autocomplete_path::FilePathCompleter,
    query::app_languages,
    zip::extract_db,
};

#[derive(Debug)]
pub struct UserInput {
    pub tr: AppTranslationInfo,
    pub source_path: SourceFile,
    pub output_path: String,
    pub mark_custom_on_export: bool,
}

#[derive(Debug)]
pub enum SourceFile {
    DB(PathBuf),
    CustomCategories(Vec<PathBuf>),
}

#[derive(Debug)]
struct ExportSpecificAnswer {
    app: App,
    source_path: SourceFile,
    mark_custom_on_export: bool,
}

/// Ask for:
///
/// - App
/// - Path to APK file (if db for this app is not cached)
/// - Translate language
/// - Where to save exported collection
///
/// If path to APK is given, extract and cache db
pub fn ask(no_cache: bool) -> Result<UserInput> {
    let what_import = SourceFileKind::select("What do you want to import?").prompt()?;

    let ExportSpecificAnswer {
        app,
        source_path: db_path,
        mark_custom_on_export,
    } = match what_import {
        SourceFileKind::Apk => ask_apk(no_cache)?,
        SourceFileKind::CustomCategories => ask_custom_categories()?,
    };

    let learn_lang = app.into();
    let tr_lang: Language =
        Select::new("Translate language:", app_languages(app).to_vec()).prompt()?;

    let output_path = Text::new("Path to exported .apkg collection:")
        .with_autocomplete(FilePathCompleter::default())
        .with_help_message(&current_dir_help())
        .with_initial_value(DEFAULT_OUTPUT_FILE)
        .with_validator(validator::validate_empty_string)
        .with_validator(validator::validate_apkg)
        .prompt()?;

    Ok(UserInput {
        tr: AppTranslationInfo {
            app,
            learn_lang,
            tr_lang,
        },
        source_path: db_path,
        output_path,
        mark_custom_on_export,
    })
}

fn ask_apk(no_cache: bool) -> Result<ExportSpecificAnswer> {
    let app: App = Select::new("App to import:", App::SUPPORTED.to_vec()).prompt()?;

    let db_path = db_cache_path(app)?;
    if !db_path.exists() || no_cache {
        let apk_path = Text::new("Path to .apk/.xapk file:")
            .with_autocomplete(FilePathCompleter::default())
            .with_help_message(&current_dir_help())
            .with_validator(validator::validate_empty_string)
            .with_validator(validator::validate_apk_xapk)
            .prompt()?;
        extract_db(app, apk_path, &db_path)?;
    } else {
        eprintln!("Using cached apk, run with --no-cache to select again");
    }

    Ok(ExportSpecificAnswer {
        app,
        source_path: SourceFile::DB(db_path),
        mark_custom_on_export: false,
    })
}

fn ask_custom_categories() -> Result<ExportSpecificAnswer> {
    let app = App::select("From which app do you import categories?").prompt()?;

    let help = format!(
        "{}. Press Enter to select, press ESC to continue",
        current_dir_help()
    );
    let cancel_render = RenderConfig {
        canceled_prompt_indicator: inquire::ui::Styled::new("<skip>")
            .with_fg(inquire::ui::Color::DarkGrey),
        ..Default::default()
    };

    // deduplicate
    let mut paths = HashSet::new();
    while let Some(p) = Text::new("Path to .reword file or directory:")
        .with_maybe_render_config(cancel_render, !paths.is_empty())
        .with_autocomplete(FilePathCompleter::default())
        .with_help_message(&help)
        .with_validator(validator::validate_empty_string)
        .with_validator(validator::validate_reword_custom_category)
        .prompt_maybe_skippable(!paths.is_empty())?
    {
        let mut p = PathBuf::from(p);
        if let Ok(real) = p.canonicalize() {
            p = real;
        }
        if p.is_file() {
            paths.insert(p);
        } else if p.is_dir() {
            let mut counter = 0;
            let mut some_files_already_added = false;
            let Some(p) = p.to_str() else {
                eprintln!(
                    "[error] expected path {:?} to be utf-8 encoded, skipping",
                    p.display()
                );
                continue;
            };
            for path in glob::glob(&format!("{p}/*.reword"))?
                .filter_map(Result::ok)
                .filter(|p| p.extension().is_some_and(|ext| ext == "reword"))
            {
                if paths.contains(&path) {
                    some_files_already_added = true;
                    continue;
                }

                paths.insert(path);
                counter += 1;
            }
            if counter == 0 && !some_files_already_added {
                eprintln!("[warn] no .reword files in {p:?}");
            } else {
                eprintln!("[info] added .reword files: {counter}");
            }
        }
    }

    let mark_custom_on_export = Confirm::new("Mark exported collection as custom?")
        .with_help_message(
            "You won't be able to join this collection with collection of built-in categories",
        )
        .with_default(false)
        .prompt()?;

    Ok(ExportSpecificAnswer {
        app,
        source_path: SourceFile::CustomCategories(paths.into_iter().collect()),
        mark_custom_on_export,
    })
}

pub fn ask_categories(categories: Vec<Category>) -> Result<Option<Vec<Category>>> {
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

fn current_dir_help() -> String {
    format!(
        "Current directory: {}",
        std::env::current_dir().unwrap().display()
    )
}

#[derive(Debug, Clone, Copy, Selectable)]
enum SourceFileKind {
    Apk,
    CustomCategories,
}

impl Display for SourceFileKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            SourceFileKind::Apk => "APK/XAPK file",
            SourceFileKind::CustomCategories => "Custom categories",
        };
        write!(f, "{s}")
    }
}

trait InquireTextExt<'ta> {
    fn with_maybe_render_config<'c: 'ta>(self, config: RenderConfig<'c>, enable: bool) -> Self;
    fn prompt_maybe_skippable(self, skip: bool) -> inquire::error::InquireResult<Option<String>>;
}

impl<'ta> InquireTextExt<'ta> for Text<'ta, '_> {
    fn with_maybe_render_config<'c: 'ta>(self, config: RenderConfig<'c>, enable: bool) -> Self {
        if enable {
            self.with_render_config(config)
        } else {
            self
        }
    }
    fn prompt_maybe_skippable(self, skip: bool) -> inquire::error::InquireResult<Option<String>> {
        if skip {
            self.prompt_skippable()
        } else {
            self.prompt().map(Some)
        }
    }
}

mod validator {
    use std::{error::Error, path::PathBuf};

    use inquire::validator::Validation;

    type ValidationResult = Result<Validation, Box<dyn Error + Send + Sync>>;

    fn validate(f: impl FnOnce() -> Result<(), &'static str>) -> ValidationResult {
        match f() {
            Ok(()) => Ok(Validation::Valid),
            Err(e) => Ok(Validation::Invalid(e.into())),
        }
    }

    fn validate_simple(ok: bool, msg: &'static str) -> ValidationResult {
        validate(|| if ok { Ok(()) } else { Err(msg) })
    }

    pub fn validate_empty_string(value: &str) -> ValidationResult {
        validate_simple(!value.is_empty(), "String is empty")
    }

    pub fn validate_apk_xapk(value: &str) -> ValidationResult {
        let p = PathBuf::from(value);
        let ok = p.is_file()
            && p.extension()
                .is_some_and(|ext| ext == "apk" || ext == "xapk");
        validate_simple(ok, "Expected .apk or .xapk file")
    }

    pub fn validate_apkg(value: &str) -> ValidationResult {
        let p = PathBuf::from(value);
        let ok = p.is_file() && p.extension().is_some_and(|ext| ext == "apkg");
        validate_simple(ok, "Expected .apkg file")
    }

    pub fn validate_reword_custom_category(value: &str) -> ValidationResult {
        let p = PathBuf::from(value);
        let ok = p.is_dir() || p.is_file() && p.extension().is_some_and(|ext| ext == "reword");
        validate_simple(ok, "Expected directory or file with .reword extension")
    }
}
