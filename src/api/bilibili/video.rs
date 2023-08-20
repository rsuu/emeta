use nanoserde::{DeJson, SerJson};

#[derive(DeJson, SerJson)]
pub struct ListEntry {
    pub tid: u32,
    pub count: u32,
    pub name: String,
}

#[derive(DeJson, SerJson)]
pub struct TList {
    pub entries: Vec<ListEntry>,
}

#[derive(DeJson, SerJson)]
pub struct Video {
    pub comment: u32,
    pub typeid: u32,
    pub play: u32,
    pub pic: String,
    pub subtitle: String,
    pub description: String,
    pub copyright: String,
    pub title: String,
    pub review: u32,
    pub author: String,
    pub mid: u32,
    pub created: u32,
    pub length: String,
    pub video_review: u32,
    pub aid: u32,
    pub bvid: String,
    pub hide_click: bool,
    pub is_pay: u32,
    pub is_union_video: u32,
    pub is_steins_gate: u32,
    pub is_live_playback: u32,
    pub is_avoided: u32,
    pub attribute: u32,
    pub is_charging_arc: bool,
    pub vt: u32,
    pub enable_vt: u32,
    pub vt_display: String,
}

#[derive(DeJson, SerJson)]
pub struct Data {
    pub list: Option<List>,
    pub page: Option<Page>,
}

#[derive(DeJson, SerJson)]
pub struct List {
    //pub tlist: TList,
    pub vlist: Vec<Video>,
}

#[derive(DeJson, SerJson)]
pub struct Page {
    pub pn: u32,
    pub ps: u32,
    pub count: u32,
}

#[derive(DeJson, SerJson)]
pub struct ApiResponse {
    pub code: u32,
    pub message: String,
    pub ttl: u32,
    pub data: Data,
}
