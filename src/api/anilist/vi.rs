use super::Query;

impl Query {
    pub const QUERY_VI_WATCH_STATTUS: &str = r#"
query ($search: String, $type: MediaType) {
        mutation ($mediaId: Int, $status: MediaListStatus) {
            SaveMediaListEntry (mediaId: $mediaId, status: $status) {
                id
            }
        }
}"#;
}
