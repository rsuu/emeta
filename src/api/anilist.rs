// REFS: https://anilist.gitbook.io/anilist-apiv2-docs

mod media;
mod medialist;
mod mutation;

use crate::meta::Progress;
use crate::*;
use media::MediaListStatus;

use nanoserde::{DeJson, SerJson};
use pico_args::Arguments;
use strum::Display;
use tabled::{
    builder::Builder,
    settings::{
        object::{Columns, Rows},
        style::Style,
        themes::Colorization,
        Alignment, Color, Modify, Width,
    },
};

#[derive(Debug)]
pub struct Anilist {
    app: App,
    req: Request,
}

impl Anilist {
    const QUERY_LS: &str = include_str!("./anilist/query_ls.gql");
    const QUERY_FD: &str = include_str!("./anilist/query_fd.gql");
    const MUT_LIST: &str = include_str!("./anilist/mut_list.gql");

    pub const API_URL: &str = "https://graphql.anilist.co/api";
    pub const AUTH_URL: &str =
        "https://anilist.co/api/v2/oauth/authorize?client_id=11549&response_type=token";

    pub fn new(app: App) -> Self {
        let url = Self::AUTH_URL;
        let token = app.anilist().token.as_str();
        if token.is_empty() {
            println!("Click \"{url}\" to start authorization process.");

            std::process::exit(0)
        }

        Self {
            req: ureq::post(Self::API_URL)
                .set("Content-Type", "application/json")
                .set("Accept", "application/json")
                .set("User-Agent", USER_AGENT)
                .set("Authorization", &format!("Bearer {}", token)),
            app,
        }
    }
}

impl Query for Anilist {
    /// USAGE: `fd.a -t a 'doraemon'`
    fn fd(&self) -> Res<()> {
        struct Args {
            ty: MediaType,

            search: String,
        }

        let Args { ty, search } = {
            let any = self.app.any().clone();
            let mut p = Arguments::from_vec(any);

            Args {
                ty: p.opt_value_from_str("-t")?.unwrap_or_default(),
                search: p.free_from_str()?,
            }
        };

        let query = format!(
            r#"
{{
    "query": "{query}",
    "variables": {{
        "search": "{search}",
        "type": "{ty}"
    }}
}}
"#,
            query = Self::QUERY_FD
        )
        .replace('\n', "");

        let res = self.req.clone().send_string(&query)?.into_string()?;
        let m = media::Top::deserialize_json(&res)?;

        let (green, red, yellow, blue) = (
            Color::FG_GREEN,
            Color::FG_RED,
            Color::FG_YELLOW,
            Color::FG_BLUE,
        );

        // Title | Type | Status | pg | id | pg
        //                             10/29
        // green | blue | any    | white| gray
        // bold                    bold
        let mut data: Vec<Vec<String>> = vec![vec![
            "Title".into(),
            "Type".into(),
            "Status".into(),
            "Progress".into(),
            "Id".into(),
        ]];
        let media::Data::Page { media } = m.data
        else {bail!("")};
        for f in media.iter() {
            debug!("{:#?}", &f);

            let title = f.title.clone().unwrap();
            let id = f.id.unwrap();
            let status = f.status.unwrap();
            let ty = "ANIME";
            let progress = "10/1";

            data.extend_from_slice(&[vec![
                title.into(),
                ty.into(),
                status.to_string(),
                progress.into(),
                id.to_string(),
            ]]);
        }

        let mut table = Builder::from(data).build();
        table
            .with(Style::re_structured_text())
            .with(Alignment::center())
            .with(
                Modify::new(Columns::single(0))
                    .with(Width::truncate(12).suffix("\u{2026}"))
                    .with(green),
            )
            .with(Modify::new(Columns::single(4)).with(Color::FG_CYAN))
            .with(Modify::new(Rows::first()).with(blue));

        Term::clear();
        println!("{}", table);

        Ok(())
    }

    fn vi(&self) -> Res<()> {
        Ok(())
    }

    /// USAGE: `set.a -i 200 -p 2 -s current`
    fn set(&self) -> Res<()> {
        struct Args {
            id: u64,
            status: MediaListStatus,
            progress: u16,
        }

        let Args {
            id,
            status,
            progress,
        } = {
            let args = self.app.any().clone();
            let mut p = pico_args::Arguments::from_vec(args);
            Args {
                id: p.opt_value_from_str("-i")?.unwrap_or_else(|| panic!("")),
                progress: p.opt_value_from_str("-p")?.unwrap_or_default(),
                status: p.opt_value_from_str("-s")?.unwrap_or_default(),
            }
        };

        let query = format!(
            r#"
{{
    "query": "{query}",
    "variables": {{
        "mediaId": {id},
        "progress": {progress},
        "status": "{status}"
    }}
}}
"#,
            query = Self::MUT_LIST
        )
        .replace('\n', "");

        println!("{}", self.req.clone().send_string(&query)?.into_string()?);

        Ok(())
    }

    /// USAGE: `ls.a -t a -s current`
    fn ls(&self) -> Res<()> {
        struct Args {
            ty: MediaType,
            username: String,
            status: MediaListStatus,
        }

        let Args {
            ty,
            username,
            status,
        } = {
            let args = self.app.any().clone();
            let mut p = pico_args::Arguments::from_vec(args);
            //dbg!(&p);
            Args {
                ty: p.opt_value_from_str("-t")?.unwrap_or(MediaType::Anime),
                username: p
                    .opt_value_from_str("-u")?
                    .unwrap_or_else(|| self.app.anilist().username.clone()),
                status: p.opt_value_from_str("-s")?.unwrap_or_default(),
            }
        };

        let query = format!(
            r#"
{{
    "query": "{query}",
    "variables": {{
        "userName": "{username}",
        "type": "{ty}",
        "status": "{status}"
    }}
}}
"#,
            query = Self::QUERY_LS
        )
        .replace('\n', "");
        //dbg!(&query);

        let res = self.req.clone().send_string(&query)?.into_string()?;
        let m = media::Top::deserialize_json(&res)?;
        //dbg!(&res);

        let (green, red, yellow, blue) = (
            Color::FG_GREEN,
            Color::FG_RED,
            Color::FG_YELLOW,
            Color::FG_BLUE,
        );

        // Title | Type | Status | pg | id | pg
        //                             10/29
        // green | blue | any    | white| gray
        // bold                    bold
        let mut data: Vec<Vec<String>> = vec![vec![
            "Title".into(),
            "Type".into(),
            "Status".into(),
            "Progress".into(),
            "Id".into(),
        ]];
        // "data": {
        // "MediaListCollection": {
        // "lists": [
        //     {
        //     "entries": [
        //         {
        //"media": {

        let media::Data::MediaListCollection { lists } = m.data
        else {bail!("")};

        let mut medias = vec![];
        for l in lists.iter() {
            for e in l.entries.iter() {
                let Some(m) = e.media.as_ref()
                else {continue;};
                medias.push(m);
            }
        }
        for f in medias.iter() {
            debug!("{:#?}", &f);

            let title = f.title.clone().unwrap();
            let id = f.id.unwrap();
            let status = f.status.unwrap();
            let ty = "ANIME";
            let progress = "10/1";

            data.extend_from_slice(&[vec![
                title.into(),
                ty.into(),
                status.to_string(),
                progress.into(),
                id.to_string(),
            ]]);
        }

        let mut table = Builder::from(data).build();
        table
            .with(Style::re_structured_text())
            .with(Alignment::center())
            .with(
                Modify::new(Columns::single(0))
                    .with(Width::truncate(12).suffix("\u{2026}"))
                    .with(green),
            )
            .with(Modify::new(Columns::single(4)).with(Color::FG_CYAN))
            .with(Modify::new(Rows::first()).with(blue));

        Term::clear();
        println!("{}", table);

        Ok(())
    }
}
