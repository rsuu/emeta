fn main() {
    let config = Config {};

    config.bilibili_conf = BiliBiliConf { feed_list: [1_i64] };
    config.anilist_conf = AnilistConf {
        username: "...",
        token: "...",
    };
}
