#[derive(Debug, Clone, Copy)]
pub enum App {
    Chinese,
    Czech,
    Dutch,
    English,
    Finnish,
    French,
    German,
    Italian,
    Japanese,
    Korean,
    Polish,
    Portuguese,
    Russian,
    Spanish,
    Turkish,
}

#[derive(Debug, Clone, Copy)]
pub enum Language {
    Japanese,
    Russian,
}

#[derive(Debug)]
pub struct TrInfo {
    pub app: App,
    pub learn_lang: Language,
    pub tr_lang: Language,
}

impl App {
    pub fn kind(&self) -> String {
        let s = match self {
            Self::Japanese => "jap",
            _ => unreachable!(),
        };
        s.to_owned()
    }
    pub fn display(&self) -> String {
        let s = match self {
            Self::Japanese => "Japanese",
            _ => unreachable!(),
        };
        s.to_owned()
    }
}

impl Language {
    pub fn dislpay(&self) -> String {
        let s = match self {
            Self::Japanese => "Japanese",
            Self::Russian => "Russian",
        };
        s.to_owned()
    }
}
