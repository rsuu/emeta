use nanoserde::{DeJson, SerJson};
use std::{collections::HashMap, default, fmt::Display, sync::Arc};

#[derive(Debug, Clone, SerJson, DeJson)]
pub struct DLsiteProductList {
    //pub last: DateTime<Utc>,
    pub limit: usize,
    pub offset: usize,
    pub works: Vec<DLsiteProduct>,
}

#[derive(Debug, Clone, SerJson, DeJson)]
pub struct DLsiteProduct {
    pub age: DLsiteProductAgeCategory,
    pub group: DLsiteProductGroup,
    pub icon: DLsiteProductIcon,
    pub id: String,
    pub title: DLsiteProductLocalizedString,
    pub ty: DLsiteProductType,
    //pub upgraded_at: Option<DateTime<Utc>>,
    //pub purchased_at: DateTime<Utc>,
    //pub registered_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, SerJson, DeJson)]
pub struct DLsiteProductLocalizedString {
    pub chinese: Option<String>,
    pub english: Option<String>,
    pub japanese: Option<String>,
    pub korean: Option<String>,
    pub taiwanese: Option<String>,
}

#[derive(Debug, Clone, SerJson, DeJson)]
pub struct DLsiteProductGroup {
    pub id: String,
    pub name: DLsiteProductLocalizedString,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, DeJson, SerJson)]
pub enum DLsiteProductType {
    #[default]
    Unknown,
    Adult,
    Doujinsji,
    Software,
    Game,
    Action,
    Adventure,
    AudioMaterial,
    Comic,
    DigitalNovel,
    Other,
    OtherGame,
    Illust,
    ImageMaterial,
    Manga,
    Anime,
    Music,
    Novel,
    Puzzle,
    Quiz,
    RolePlaying,
    Gekiga, // See https://en.wikipedia.org/wiki/Gekiga
    Simulation,
    Voice,
    Shooter,
    Tabletop,
    Utility,
    Typing,
    SexualNovel,
    VoiceComic,
}

impl Display for DLsiteProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use DLsiteProductType::*;
        write!(f, "{}", {
            match self {
                Adult => "ADULT",
                Doujinsji => "DOUJINSJI",
                Software => "SOFTWARE",
                Game => "GAME",
                Action => "ACTION",
                Adventure => "ADVENTURE",
                AudioMaterial => "AUDIOMATERIAL",
                Comic => "COMIC",
                DigitalNovel => "DIGITALNOVEL",
                Other => "OTHER",
                OtherGame => "OTHERGAME",
                Illust => "ILLUST",
                ImageMaterial => "IMAGEMATERIAL",
                Manga => "MANGA",
                Anime => "ANIME",
                Music => "MUSIC",
                Novel => "NOVEL",
                Puzzle => "PUZZLE",
                Quiz => "QUIZ",
                RolePlaying => "ROLEPLAYING",
                Gekiga => "GEKIGA",
                Simulation => "SIMULATION",
                Voice => "VOICE",
                Shooter => "SHOOTER",
                Tabletop => "TABLETOP",
                Utility => "UTILITY",
                Typing => "TYPING",
                SexualNovel => "SEXUALNOVEL",
                VoiceComic => "VOICECOMIC",
                Unknown => "UNKNOWN",
            }
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, DeJson, SerJson)]
pub enum DLsiteProductAgeCategory {
    #[default]
    Unknown,
    All,
    R15,
    R18,
}

impl Display for DLsiteProductAgeCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> core::fmt::Result {
        use DLsiteProductAgeCategory::*;
        write!(f, "{}", {
            match self {
                All => "All",
                R15 => "R15",
                R18 => "R18",
                Unknown => " UNKNOWN",
            }
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SerJson, DeJson)]
pub struct DLsiteProductIcon {
    pub main: String,

    pub small: String,
}

#[derive(Debug, Clone, SerJson, DeJson)]
pub struct DLsiteProductDetail {
    pub image: DLsiteProductDetailImage,
    pub contents: Vec<DLsiteProductDetailContent>,
}

#[derive(Debug, Clone, SerJson, DeJson)]
pub struct DLsiteProductDetailImage {
    pub file_name: String,
    pub file_size: String,
    pub url: String,
}

#[derive(Debug, Clone, SerJson, DeJson)]
pub struct DLsiteProductDetailContent {
    pub file_name: String,
    pub file_size: String,
}

//pub async fn login(
//    username: impl AsRef<str>,
//    password: impl AsRef<str>,
//) -> Result<Arc<CookieStoreMutex>> {
//    let cookie_store = Arc::new(CookieStoreMutex::new(CookieStore::default()));
//    let client = ClientBuilder::new()
//        .cookie_store(true)
//        .cookie_provider(cookie_store.clone())
//        .build()?;
//
//    client
//        .get("https://www.dlsite.com/maniax/login/=/skip_register/1")
//        .send()
//        .await?;
//    client.get("https://login.dlsite.com/login").send().await?;
//
//    // We must ignore the response of this login request here.
//    // The DLsite always responds with normal 200 status even if the login has been failed.
//    client
//        .post("https://login.dlsite.com/login")
//        .form(&[
//            ("login_id", username.as_ref()),
//            ("password", password.as_ref()),
//            ("_token", &{
//                let cookie = cookie_store
//                    .lock()
//                    .unwrap()
//                    .get("login.dlsite.com", "/", "XSRF-TOKEN")
//                    .ok_or_else(|| Error::DLsiteCookieNotFound {
//                        cookie_domain: "login.dlsite.com".to_owned(),
//                        cookie_path: "/".to_owned(),
//                        cookie_name: "XSRF-TOKEN".to_owned(),
//                    })?
//                    .value()
//                    .to_owned();
//                cookie
//            }),
//        ])
//        .send()
//        .await?;
//
//    Ok(cookie_store)
//}
//
//pub async fn get_product_count(cookie_store: Arc<CookieStoreMutex>) -> Result<usize> {
//    let client = ClientBuilder::new()
//        .cookie_store(true)
//        .cookie_provider(cookie_store)
//        .build()?;
//    let response = client
//        .get("https://play.dlsite.com/api/product_count")
//        .send()
//        .await?;
//
//    // The body of the response will be a valid json if the login has been succeed.
//    match response.json::<HashMap<String, usize>>().await {
//        Ok(product_count) => Ok(product_count.get("user").cloned().unwrap_or(0)),
//        Err(..) => Err(Error::DLsiteNotAuthenticated),
//    }
//}
//
//pub async fn get_product(
//    cookie_store: Arc<CookieStoreMutex>,
//    page: usize,
//) -> Result<Vec<DLsiteProduct>> {
//    let client = ClientBuilder::new()
//        .cookie_store(true)
//        .cookie_provider(cookie_store)
//        .build()?;
//    let response = client
//        .get(format!(
//            "https://play.dlsite.com/api/purchases?page={}",
//            page
//        ))
//        .send()
//        .await?;
//
//    // The body of the response will be a valid json if the login has been succeed.
//    match response.json::<DLsiteProductList>().await {
//        Ok(product_list) => Ok(product_list.works),
//        Err(..) => Err(Error::DLsiteNotAuthenticated),
//    }
//}
//
//pub async fn get_product_details(
//    cookie_store: Arc<CookieStoreMutex>,
//    product_id: impl AsRef<str>,
//) -> Result<Vec<DLsiteProductDetail>> {
//    let client = ClientBuilder::new()
//        .cookie_store(true)
//        .cookie_provider(cookie_store)
//        .build()?;
//    let response = client
//        .get(format!(
//            "https://www.dlsite.com/maniax/api/=/product.json?workno={}",
//            product_id.as_ref()
//        ))
//        .send()
//        .await?;
//
//    // The body of the response will be a valid json if the login has been succeed.
//    match response.json::<Vec<DLsiteProductDetail>>().await {
//        Ok(details) => Ok(details),
//        Err(err) => {
//            println!("{:#?}", err);
//            Err(Error::DLsiteNotAuthenticated)
//        }
//    }
//}
