use nanoserde::{DeJson, SerJson};
use std::collections::HashMap;

pub type KeyVal = HashMap<String, String>;

#[derive(Debug, DeJson, SerJson)]
pub struct MediaInfo {
    result: String,
    response: String,
    data: Data,
}

#[derive(Debug, DeJson, SerJson)]
pub struct Tag {
    id: String,
    #[nserde(rename = "type")]
    ty: String,
    attributes: TagAttributes,
    relationships: Vec<String>,
}

#[derive(Debug, DeJson, SerJson)]
pub struct TagAttributes {
    name: KeyVal,
    //description: Option<String>,
    group: String,
    //version: u32,
}

#[derive(Debug, DeJson, SerJson)]
struct Description {}

#[derive(Debug, DeJson, SerJson)]
pub struct Attributes {
    title: KeyVal,
    #[nserde(rename = "altTitles")]
    alt_titles: Vec<KeyVal>,
    description: KeyVal,
    #[nserde(rename = "isLocked")]
    is_locked: bool,
    //links,
    #[nserde(rename = "originalLanguage")]
    ol: String,
    #[nserde(rename = "lastVolume")]
    last_volume: String,
    #[nserde(rename = "lastChapter")]
    last_chapter: String,
    #[nserde(rename = "publicationDemographic")]
    pd: String,
    status: String,
    year: u32,
    #[nserde(rename = "contentRating")]
    cr: String,
    tags: Vec<Tag>,
    state: String,
    #[nserde(rename = "chapterNumbersResetOnNewVolume")]
    cnrv: bool,
    #[nserde(rename = "createdAt")]
    created_at: String,
    #[nserde(rename = "updatedAt")]
    updated_at: String,
    //version: u32,
    #[nserde(rename = "availableTranslatedLanguages")]
    atl: Vec<String>,
    #[nserde(rename = "latestUploadedChapter")]
    luc: String,
}

#[derive(Debug, DeJson, SerJson)]
pub struct Data {
    id: String,
    #[nserde(rename = "type")]
    ty: String,
    attributes: Attributes,
    //relationships: Vec<Relationship>,
}

#[derive(Debug, DeJson, SerJson)]
pub struct Relationship {
    id: String,
    #[nserde(rename = "type")]
    ty: String,
    related: String,
}
