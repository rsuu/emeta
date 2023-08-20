#[derive(Debug, Default, Clone, PartialEq)]
pub struct Title {
    pub native: String,
    pub romaji: Option<String>,
    pub english: Option<String>,
    pub user_preferred: Option<String>,
}
