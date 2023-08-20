use super::MangaDex;

impl MangaDex {
    pub const API_URL: &str = "https://api.mangadex.org";
    pub const CDN_URL: &str = "https://uploads.mangadex.org";
    pub const AT_HOME_POST_URL: &str = "https://api.mangadex.network/report";

    //    pub const API_MANGA_URL: String = format!("{}/manga", Self::API_URL);
    //    pub const API_CHAPTER_URL: String = format!("{}/chapter", Self::API_URL);
    //    pub const API_LIST_URL: String = format!("{}/list", Self::API_URL);

    pub const PREFIX_ID_SEARCH: &str = "id:";
    pub const PREFIX_CH_SEARCH: &str = "ch:";
    pub const PREFIX_GRP_SEARCH: &str = "grp:";
    pub const PREFIX_AUTH_SEARCH: &str = "author:";
    pub const PREFIX_USR_SEARCH: &str = "usr:";
    pub const PREFIX_LIST_SEARCH: &str = "list:";
}
