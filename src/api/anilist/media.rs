use nanoserde::{DeJson, SerJson};
use std::{default, fmt::Display, str::FromStr};
use strum::Display;

#[derive(Debug, DeJson)]
pub struct Top {
    pub data: Data,
}

#[derive(Debug, DeJson)]
pub enum Data {
    Page { media: Vec<Media> },
    MediaListCollection { lists: Vec<List> },
}

#[derive(Debug, DeJson)]
pub struct List {
    pub entries: Vec<Entry>,
}

#[derive(Debug, DeJson)]
pub struct Entry {
    pub media: Option<Media>,
}

#[derive(Debug, DeJson)]
pub struct Media {
    pub id: Option<u32>,
    pub title: Option<Title>,
    #[nserde(rename = "idMal")]
    id_mal: Option<u32>,
    #[nserde(rename = "startDate")]
    start_date: Option<TinyDate>,
    #[nserde(rename = "endDate")]
    end_date: Option<TinyDate>,
    season: Option<MediaSeason>,
    #[nserde(rename = "seasonYear")]
    season_year: Option<u32>,
    #[nserde(rename = "type")]
    ty: Option<String>,
    //    format: Option<MediaFormat>,
    pub status: Option<MediaStatus>,
    episodes: Option<u32>,
    duration: Option<u32>,
    chapters: Option<u32>,
    volumes: Option<u32>,
    #[nserde(rename = "isAdult")]
    is_adult: Option<bool>,
    genre: Option<String>,
    tag: Option<String>,
    //    minimumTagRank: Option<u32>,
    //    tagCategory: Option<String>,
    //    onList: Option<bool>,
    //    licensedBy: Option<String>,
    //    licensedById: Option<u32>,
    //    averageScore: Option<u32>,
    //    popularity: Option<u32>,
    //    source: Option<MediaSource>,
    //    countryOfOrigin: Option<CountryCode>,
    //    isLicensed: Option<bool>,
    //    search: Option<String>,
    //    id_not: Option<u32>,
    //    id_in: Option<Vec<u32>>,
    //    id_not_in: Option<Vec<u32>>,
    //    idMal_not: Option<u32>,
    //    idMal_in: Option<Vec<u32>>,
    //    idMal_not_in: Option<Vec<u32>>,
    //    startDate_greater: Option<TinyDate>,
    //    startDate_lesser: Option<TinyDate>,
    //    startDate_like: Option<String>,
    //    endDate_greater: Option<TinyDate>,
    //    endDate_lesser: Option<TinyDate>,
    //    endDate_like: Option<String>,
    //    format_in: Option<Vec<MediaFormat>>,
    //    format_not: Option<MediaFormat>,
    //    format_not_in: Option<Vec<MediaFormat>>,
    //    status_in: Option<Vec<MediaStatus>>,
    //    status_not: Option<MediaStatus>,
    //    status_not_in: Option<Vec<MediaStatus>>,
    //    episodes_greater: Option<u32>,
    //    episodes_lesser: Option<u32>,
    //    duration_greater: Option<u32>,
    //    duration_lesser: Option<u32>,
    //    chapters_greater: Option<u32>,
    //    chapters_lesser: Option<u32>,
    //    volumes_greater: Option<u32>,
    //    volumes_lesser: Option<u32>,
    //    genre_in: Option<Vec<String>>,
    //    genre_not_in: Option<Vec<String>>,
    //    tag_in: Option<Vec<String>>,
    //    tag_not_in: Option<Vec<String>>,
    //    tagCategory_in: Option<Vec<String>>,
    //    tagCategory_not_in: Option<Vec<String>>,
    //    licensedBy_in: Option<Vec<String>>,
    //    licensedById_in: Option<Vec<u32>>,
    //    averageScore_not: Option<u32>,
    //    averageScore_greater: Option<u32>,
    //    averageScore_lesser: Option<u32>,
    //    popularity_not: Option<u32>,
    //    popularity_greater: Option<u32>,
    //    popularity_lesser: Option<u32>,
    //    source_in: Option<Vec<MediaSource>>,
    //    sort: Option<Vec<MediaSort>>,
}

#[derive(Debug, DeJson, Clone)]
pub struct Title {
    romaji: Option<String>,
    english: Option<String>,
    native: Option<String>,
    #[nserde(rename = "userPreferred")]
    user_preferred: Option<String>,
}

#[derive(Debug, DeJson)]
pub struct TinyDate {
    year: Option<u32>,
    month: Option<u32>,
    day: Option<u32>,
}

#[derive(Debug, DeJson)]
pub enum MediaSeason {
    #[nserde(rename = "WINTER")]
    Winter,
    #[nserde(rename = "SPRING")]
    Spring,
    #[nserde(rename = "SUMMER")]
    Summer,
    #[nserde(rename = "FALL")]
    Fall,
}

#[derive(Debug, DeJson)]
pub enum MediaFormat {
    #[nserde(rename = "TV")]
    TV,
    #[nserde(rename = "TV_SHORT")]
    TVShort,
    #[nserde(rename = "MOVIE")]
    Movie,
    #[nserde(rename = "SPECIAL")]
    Special,
}

#[derive(Debug, Copy, Clone, DeJson, Display)]
pub enum MediaStatus {
    #[nserde(rename = "FINISHED")]
    Finished,
    #[nserde(rename = "CANCELLED")]
    Cancelled,
    #[nserde(rename = "HIATUS")]
    Hiatus,
    #[nserde(rename = "NOT_YET_RELEASED")]
    NotYetReleased,
    #[nserde(rename = "RELEASING")]
    Releasing,
}

#[derive(Debug, DeJson)]
pub enum MediaSource {
    #[nserde(rename = "ORIGINAL")]
    Original,
    #[nserde(rename = "MANGA")]
    Manga,
    #[nserde(rename = "LIGHT_NOVEL")]
    LightNovel,
    #[nserde(rename = "VISUAL_NOVEL")]
    VisualNovel,
    #[nserde(rename = "VIDEO_GAME")]
    VideoGame,
}

#[derive(Debug, DeJson)]
pub enum CountryCode {
    #[nserde(rename = "JP")]
    Japan,
    #[nserde(rename = "KR")]
    Korea,
    #[nserde(rename = "US")]
    UnitedStates,
    #[nserde(rename = "CN")]
    China,
}

#[derive(Debug, DeJson)]
pub enum MediaSort {
    Unknown,
}

#[derive(Debug, SerJson, DeJson, Default)]
pub enum MediaListStatus {
    #[default]
    Unknown,
    #[nserde(rename = "CURRENT")]
    Current,
    #[nserde(rename = "PLANNING")]
    Planning,
    #[nserde(rename = "COMPLETED")]
    Completed,
    #[nserde(rename = "DROPPED")]
    Dropped,
    #[nserde(rename = "PAUSED")]
    Paused,
    #[nserde(rename = "REPEATING")]
    Repeating,
}

impl FromStr for MediaListStatus {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MediaListStatus::*;
        Ok(match s.to_ascii_uppercase().as_str() {
            "CURRENT" => Current,
            "PLANNING" => Planning,
            "COMPLETED" => Completed,
            "DROPPED" => Dropped,
            "PAUSED" => Paused,
            "REPEATING" => Repeating,
            _ => Unknown,
        })
    }
}

impl std::fmt::Display for MediaListStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use MediaListStatus::*;

        write!(f, "{}", {
            match self {
                Current => "CURRENT",
                Planning => "PLANNING",
                Completed => "COMPLETED",
                Dropped => "DROPPED",
                Paused => "PAUSED",
                Repeating => "REPEATING",
                Unknown => "Unknown",
            }
        })
    }
}

impl Into<String> for Title {
    fn into(self) -> String {
        if let Some(native) = self.native {
            native
        } else if let Some(romaji) = self.romaji {
            romaji
        } else if let Some(english) = self.english {
            english
        } else {
            format!("")
        }
    }
}
