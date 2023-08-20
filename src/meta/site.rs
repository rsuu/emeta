use esyn::EsynDe;
use std::{default, fmt::Display, str::FromStr};

#[derive(Debug, Clone, Copy, EsynDe, Default)]
pub enum Site {
    ///
    #[default]
    Anilist,
    ///
    MangaDex,
    EHentai,
    BiliBili,
    /// ?
    Fs,
}

impl Display for Site {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Anilist => write!(f, "ANILIST"),
            Self::MangaDex => write!(f, "MANGADEX"),
            Self::EHentai => write!(f, "EHENTAI"),
            Self::Fs => write!(f, "FS"),
            Self::BiliBili => write!(f, "BILIBILI"),
        }
    }
}

impl FromStr for Site {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_ascii_uppercase().as_str() {
            "A" | "ANILIST" => Self::Anilist,
            "M" | "MANGADEX" => Self::MangaDex,
            "E" | "EHENTAI" => Self::EHentai,
            "F" | "FS" => Self::Fs,
            "BB" | "BILIBILI" => Self::BiliBili,
            _ => Default::default(),
        })
    }
}
