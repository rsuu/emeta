mod video;

use crate::*;
use nanoserde::DeJson;

#[derive(Debug)]
pub struct BiliBili {
    app: App,
}

impl BiliBili {
    pub fn new(app: App) -> Self {
        Self { app }
    }
}

impl Query for BiliBili {
    // REFS: https://github.com/DIYgod/RSSHub/blob/fcc0fab4d01f565016dc60a391ebec2eca043f54/lib/v2/bilibili/video.js
    fn ls(&self) -> crate::Res<()> {
        use video::*;

        struct Args {
            uid: Option<i64>,
        }

        let Args { uid } = {
            let any = self.app.any().clone();
            let mut p = pico_args::Arguments::from_vec(any);

            Args {
                uid: p.opt_value_from_str("--id")?,
            }
        };

        // cli
        if let Some(uid) = uid {
            let url = ls_gen_url(uid);
            let res = ureq::get(&url).call()?.into_string()?;
            let v = ApiResponse::deserialize_json(&res)?;
            debug!("{}", &v.code);

            let list = v.data.list.unwrap();
            for f in list.vlist.iter() {
                dbg!(&f.title, &f.created, &f.author, &f.bvid);
            }

            return Ok(());
        }

        // config
        for uid in self.app.bilibili().feed_list.iter() {
            let url = ls_gen_url(*uid);
            let res = ureq::get(&url).call()?.into_string()?;
            let v = ApiResponse::deserialize_json(&res)?;
            debug!("{}", &v.code);

            let list = v.data.list.unwrap();
            for f in list.vlist.iter() {
                dbg!(&f.title, &f.created, &f.author, &f.bvid);
            }
        }

        Ok(())
    }
}

fn ls_gen_url(uid: i64) -> String {
    let params =format!("mid={uid}&ps=30&tid=0&pn=1&keyword=&order=pubdate&platform=web&web_location=1550101&order_avoided=true");

    format!("https://api.bilibili.com/x/space/wbi/arc/search?{params}")
}
