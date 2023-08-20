use esyn::EsynDe;
use std::{default, fmt::Display, str::FromStr};

// ========== Cli ==========
#[derive(Debug, Default, Clone, Copy, EsynDe)]
pub enum MediaType {
    #[default]
    Unknown = 0,
    Anime = 1,
    Cg = 2,
    Comic = 3,
    Doujinshi = 4,
    Game = 5,
    LightNovel = 6,
    LiveAction = 7,
    Manga = 8,
    MultimediaProject = 9,
    Novel = 10,
    Original = 11,
    Other = 12,
    PictureBook = 13,
    VideoGame = 14,
    VisualNovel = 15,
    WebNovel = 16,
}

impl FromStr for MediaType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MediaType::*;

        Ok(match s.to_ascii_uppercase().as_str() {
            "A" | "ANIME" => Anime,
            "CG" => Cg,
            "C" | "COMIC" => Comic,
            "DOUJINSUU" => Doujinshi,
            "G" | "GAME" => Game,
            "LA" => LiveAction,
            "LN" => LightNovel,
            "M" | "MANGA" => Manga,
            "MP" => MultimediaProject,
            "N" | "NOVEL" => Novel,
            "O" | "ORIGINAU" => Original,
            "OTHER" => Other,
            "PB" => PictureBook,
            "VG" => VideoGame,
            "VN" => VisualNovel,
            "WN" => WebNovel,

            _ => Default::default(),
        })
    }
}

impl Display for MediaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use MediaType::*;

        write!(f, "{}", {
            match self {
                Anime => "ANIME",
                Cg => "CG",
                Comic => "COMIC",
                Doujinshi => "DOUJINSUU",
                Game => "GAME",
                LiveAction => "LA",
                LightNovel => "LN",
                Manga => "MANGA",
                MultimediaProject => "MP",
                Novel => "NOVEL",
                Original => "ORIGINAU",
                Other => "OTHER",
                PictureBook => "PB",
                VideoGame => "VG",
                VisualNovel => "VN",
                WebNovel => "WN",
                Unknown => "UNKNOWN",
            }
        })
    }
}
