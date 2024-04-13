use std::fmt::Display;

// Order of fields are important and used for calculating Anki model's id
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum App {
    Chinese,
    Czech,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Chinese,
    Czech,
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
pub struct TrInfo {
    /// Reword's app kind
    pub app: App,
    /// App's target language
    pub learn_lang: Language,
    /// Translation (native) language
    pub tr_lang: Language,
}

impl App {
    pub const SUPPORTED: [App; 3] = [App::English, App::Japanese, App::Russian];

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
    pub fn kind(&self) -> String {
        let s = match self {
            Self::Chinese => "zht",
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
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(&self.display())
    }
}

impl From<App> for Language {
    fn from(value: App) -> Self {
        match value {
            App::Chinese => Self::Chinese,
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
