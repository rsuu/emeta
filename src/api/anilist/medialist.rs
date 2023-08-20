use nanoserde::{DeJson, SerJson};

#[derive(Debug, SerJson, DeJson)]
pub struct MediaListQuery {
    query: Query,
    variables: Variables,
}

#[derive(Debug, SerJson, DeJson)]
pub struct Query {
    MediaList: MediaListArgs,
}

#[derive(Debug, SerJson, DeJson)]
pub struct MediaListArgs {
    id: Option<i32>,
    userId: Option<i32>,
    userName: Option<String>,
    #[nserde(rename = "type")]
    mediaType: Option<String>,
    status: Option<String>,
    mediaId: Option<i32>,
    isFollowing: Option<bool>,
    notes: Option<String>,
    startedAt: Option<FuzzyDateInput>,
    completedAt: Option<FuzzyDateInput>,
    compareWithAuthList: Option<bool>,
    userId_in: Option<Vec<i32>>,
    status_in: Option<Vec<String>>,
    status_not_in: Option<Vec<String>>,
    status_not: Option<String>,
    mediaId_in: Option<Vec<i32>>,
    mediaId_not_in: Option<Vec<i32>>,
    notes_like: Option<String>,
    startedAt_greater: Option<FuzzyDateInput>,
    startedAt_lesser: Option<FuzzyDateInput>,
    startedAt_like: Option<String>,
    completedAt_greater: Option<FuzzyDateInput>,
    completedAt_lesser: Option<FuzzyDateInput>,
    completedAt_like: Option<String>,
    sort: Option<Vec<MediaListSort>>,
}

#[derive(Debug, SerJson, DeJson)]
pub struct Variables {
    id: Option<i32>,
}

#[derive(Debug, SerJson, DeJson)]
pub struct FuzzyDateInput {
    year: Option<i32>,
    month: Option<i32>,
    day: Option<i32>,
}

#[derive(Debug, SerJson, DeJson)]
pub struct MediaListSort {
    #[nserde(rename = "type")]
    sortType: String,
    order: String,
}
