//! anilist mutation
use super::media::*;
use nanoserde::{DeJson, SerJson};

#[derive(Debug, SerJson, DeJson)]
pub struct SaveMediaListEntry {
    id: i32,
    mediaId: i32,
    status: MediaListStatus,
    score: Option<f64>,
    scoreRaw: i32,
    progress: i32,
    progressVolumes: i32,
    repeat: i32,
    priority: i32,
    private: bool,
    notes: String,
    hiddenFromStatusLists: bool,
    customLists: Vec<String>,
    advancedScores: Vec<f64>,
    startedAt: Option<FuzzyDateInput>,
    completedAt: Option<FuzzyDateInput>,
}

#[derive(Debug, SerJson, DeJson)]
pub struct FuzzyDateInput {
    year: Option<i32>,
    month: Option<i32>,
    day: Option<i32>,
}
