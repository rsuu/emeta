use esyn::EsynDe;
use std::str::FromStr;
use std::{default, fmt};

#[derive(Debug, Default, Clone, Copy, EsynDe)]
pub enum Format {
    #[default]
    Unknown = 0,
    Tv,
    TvShort,
    Movie,
    Special,
    Ova,
    Ona,
    Music,
    Manga,
    Novel,
    OneShot,
}

impl FromStr for Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Format::*;

        Ok(match s.to_uppercase().as_str() {
            "TV" => Format::Tv,
            "TV-SHORT" => Format::TvShort,
            "MOVIE" => Format::Movie,
            "SPECIAL" => Format::Special,
            "OVA" => Format::Ova,
            "ONA" => Format::Ona,
            "MUSIC" => Format::Music,
            "MANGA" => Format::Manga,
            "NOVEL" => Format::Novel,
            "ONE-SHOT" => Format::OneShot,
            _ => Unknown,
        })
    }
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Format::*;

        write!(f, "{}", {
            match self {
                Tv => "TV",
                TvShort => "TV-SHORT",
                Movie => "MOVIE",
                Special => "SPECIAL",
                Ova => "OVA",
                Ona => "ONA",
                Music => "MUSIC",
                Manga => "MANGA",
                Novel => "NOVEL",
                OneShot => "ONE-SHOT",
                Unknown => "UNKNOWN",
            }
        })
    }
}
