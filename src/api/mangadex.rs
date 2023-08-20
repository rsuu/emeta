// REFS: https://api.mangadex.org/docs/swagger.html
// TODO: oauth

mod consts;
mod meta;

use crate::{bail, debug, meta::*, App, Query, Res, USER_AGENT};
use nanoserde::DeJson;
use std::fmt::Display;

#[derive(Debug)]
pub struct MangaDex {
    app: App,
}

impl MangaDex {
    pub fn new(app: App) -> Self {
        Self { app }
    }
}

impl Query for MangaDex {
    // `-s m cat -- --ty m --id 'e36da8b0-feca-46dd-9120-d5dc2f3feae8'`
    fn cat(&self) -> Res<()> {
        use meta::MediaInfo;

        struct Args {
            ty: Option<MediaType>,
            id: Option<String>,
        }

        let Args { ty, id } = {
            let any = self.app.any().clone();
            let mut p = pico_args::Arguments::from_vec(any);

            Args {
                ty: p.opt_value_from_str("--ty")?,
                id: p.opt_value_from_str("--id")?,
            }
        };

        let res = MediaInfo::deserialize_json(&{
            let ty = ty
                .as_ref()
                .unwrap_or_else(|| self.app.media_type())
                .to_string();
            let id = id.as_ref().unwrap();
            let req = format!("{}/{ty}/{id}", Self::API_URL);

            ureq::get(&req).call()?.into_string()?
        })?;

        println!("{:#?}", &res);

        Ok(())
    }

    // `-s m dl --id '28753fc8-d9bf-46f6-8499-217bd66b9b3f' -t m`
    fn dl(&self) -> Res<()> {
        use crate::*;
        use nanoserde::DeJson;
        use std::{
            fs::{write, File},
            io::Write,
        };

        struct Args {
            id: Option<String>,
        }

        #[derive(DeJson)]
        struct MangaChapter {
            result: String,
            //baseUrl: String,
            chapter: ChapterData,
        }

        #[derive(DeJson)]
        struct ChapterData {
            hash: String,
            data: Vec<String>,
            //dataSaver: Vec<String>,
        }

        let args = {
            let any = self.app.any().clone();
            let mut p = pico_args::Arguments::from_vec(any);
            Args {
                id: p.opt_value_from_str("--id")?,
            }
        };
        let Args { id:Some(id) }=args
        else {bail!("")};

        let MangaChapter {
            chapter: ChapterData { hash, data },
            result,
        } = {
            let url = format!("{host}/at-home/server/{id}", host = MangaDex::API_URL);
            let res = ureq::get(&url).call()?.into_string()?;

            MangaChapter::deserialize_json(&res)?
        };
        if result != "ok" {
            bail!("")
        }

        for page in data.iter() {
            let url = format!("{host}/data/{hash}/{page}", host = MangaDex::CDN_URL);
            let mut f = File::create(page)?;
            let mut buf = vec![];

            ureq::get(&url)
                .call()?
                .into_reader()
                .read_to_end(&mut buf)?;
            //f.write_all(&buf)?;

            todo!();
            println!("OK: {page}");
        }

        Ok(())
    }
}
