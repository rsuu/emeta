#[derive(Debug, Default, Clone, PartialEq)]
pub struct Tag {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub category: String,
    pub rank: u64,
    pub is_general_spoiler: bool,
    pub is_media_spoiler: bool,
    pub is_adult: bool,
    pub user_id: Option<u64>,
}
