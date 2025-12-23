use std::fmt::Display;

use inquire_derive::Selectable;

// Order of fields are important and used for calculating Anki model's id
#[derive(Debug, Clone, Copy, Selectable)]
pub enum App {
    Chinese,
    Czech,
    /// German
    Deutsch,
    Dutch,
    English,
    Finnish,
    French,
    Italian,
    Japanese,
    Korean,
    Polish,
    Portuguese,
    Russian,
    Spanish,
    Turkish,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Selectable)]
pub enum Language {
    ChineseSimplified,
    ChineseTraditional,
    Czech,
    /// German
    Deutsch,
    Dutch,
    English,
    Finnish,
    French,
    Italian,
    Japanese,
    Korean,
    Polish,
    Portuguese,
    Russian,
    Spanish,
    Turkish,
}

#[derive(Debug, Clone)]
pub struct AppTranslationInfo {
    /// Reword's app kind
    pub app: App,
    /// App's target language
    pub learn_lang: Language,
    /// Translation (native) language
    pub tr_lang: Language,
}

impl App {
    pub const SUPPORTED: &[App] = &[
        App::English,
        App::Deutsch,
        App::Finnish,
        App::Japanese,
        App::Russian,
    ];

    pub fn kind(&self) -> String {
        let s = match self {
            Self::Chinese => "ch",
            Self::Czech => "cz",
            Self::Deutsch => "deu",
            Self::Dutch => "du",
            Self::English => "eng",
            Self::Finnish => "fin",
            Self::French => "fr",
            Self::Italian => "it",
            Self::Japanese => "jap",
            Self::Korean => "kor",
            Self::Polish => "pol",
            Self::Portuguese => "por",
            Self::Russian => "rus",
            Self::Spanish => "sp",
            Self::Turkish => "tur",
        };
        s.to_owned()
    }
    pub fn display(&self) -> String {
        let s = match self {
            Self::Chinese => "Chinese",
            Self::Czech => "Czech",
            Self::Deutsch => "Deutsch",
            Self::Dutch => "Dutch",
            Self::English => "English",
            Self::Finnish => "Finnish",
            Self::French => "French",
            Self::Italian => "Italian",
            Self::Japanese => "Japanese",
            Self::Korean => "Korean",
            Self::Polish => "Polish",
            Self::Portuguese => "Portuguese",
            Self::Russian => "Russian",
            Self::Spanish => "Spanish",
            Self::Turkish => "Turkish",
        };
        s.to_owned()
    }
    pub fn name(&self) -> String {
        match self {
            Self::English => "ReWord: Learn English Language".to_string(),
            Self::Japanese => "Learn Japanese JLPT vocabulary".to_string(),
            Self::Portuguese => "Learn Portuguese with ReWord".to_string(),
            _ => format!("Learn {} with flashcards", self.display()),
        }
    }
}

impl Display for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(&self.name())
    }
}

impl Language {
    /// Name of language column in DB
    pub fn db_name(&self) -> String {
        let s = match self {
            Self::ChineseSimplified => "zhs",
            Self::ChineseTraditional => "zht",
            Self::Czech => "cz",
            Self::Deutsch => "deu",
            Self::Dutch => "du",
            Self::English => "eng",
            Self::Finnish => "fin",
            Self::French => "fra",
            Self::Italian => "ita",
            Self::Japanese => "jpn",
            Self::Korean => "kor",
            Self::Polish => "pol",
            Self::Portuguese => "por",
            Self::Russian => "rus",
            Self::Spanish => "spa",
            Self::Turkish => "tur",
        };
        s.to_owned()
    }
    pub fn from_db_name(s: &str) -> Self {
        match s {
            "zhs" => Self::ChineseSimplified,
            "zht" => Self::ChineseTraditional,
            "cz" => Self::Czech,
            "deu" => Self::Deutsch,
            "du" => Self::Dutch,
            "eng" => Self::English,
            "fin" => Self::Finnish,
            "fra" => Self::French,
            "ita" => Self::Italian,
            "jpn" => Self::Japanese,
            "kor" => Self::Korean,
            "pol" => Self::Polish,
            "por" => Self::Portuguese,
            "rus" => Self::Russian,
            "spa" => Self::Spanish,
            "tur" => Self::Turkish,
            _ => unreachable!("unknown db_name {s:?}"),
        }
    }
    pub fn display(&self) -> String {
        let s = match self {
            Self::ChineseSimplified => "Chinese",
            Self::ChineseTraditional => "Chinese traditional",
            Self::Czech => "Czech",
            Self::Deutsch => "Deutsch",
            Self::Dutch => "Dutch",
            Self::English => "English",
            Self::Finnish => "Finnish",
            Self::French => "French",
            Self::Italian => "Italian",
            Self::Japanese => "Japanese",
            Self::Korean => "Korean",
            Self::Polish => "Polish",
            Self::Portuguese => "Portuguese",
            Self::Russian => "Russian",
            Self::Spanish => "Spanish",
            Self::Turkish => "Turkish",
        };
        s.to_owned()
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(&self.display())
    }
}

impl From<App> for Language {
    fn from(value: App) -> Self {
        match value {
            App::Chinese => Self::ChineseSimplified,
            App::Czech => Self::Czech,
            App::Deutsch => Self::Deutsch,
            App::Dutch => Self::Dutch,
            App::English => Self::English,
            App::Finnish => Self::Finnish,
            App::French => Self::French,
            App::Italian => Self::Italian,
            App::Japanese => Self::Japanese,
            App::Korean => Self::Korean,
            App::Polish => Self::Polish,
            App::Portuguese => Self::Portuguese,
            App::Russian => Self::Russian,
            App::Spanish => Self::Spanish,
            App::Turkish => Self::Turkish,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lang_from_db_name() {
        use crate::info::__inquire_enum_choice_for_language::Variants;
        for lang in Language::VARIANTS {
            assert_eq!(Language::from_db_name(lang.db_name().as_str()), *lang);
        }
    }
}
