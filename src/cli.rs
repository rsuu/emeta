use crate::{api, bail, debug, info, meta::MediaType, Res, Site, WatchStatus, VERSION};
use esyn::{Esyn, EsynDe};
use std::{default, ffi::OsString, fs::File, io::Read, path::PathBuf, str::FromStr};

#[derive(Debug)]
pub struct App {
    pub config: Config,
    pub cli: Cli,
}

#[derive(Debug, EsynDe, Default)]
pub struct Config {
    pub setting: Setting,
    pub anilist_conf: AnilistConf,
    pub bilibili_conf: BiliBiliConf,
    pub mangadex_conf: MangadexConf,
}

#[derive(Debug, EsynDe, Default)]
pub struct Setting {
    pub media_type: MediaType,
    pub filter_watch_status: WatchStatus,
    pub cmd: String,
}

#[derive(Debug, EsynDe, Default)]
pub struct AnilistConf {
    pub username: String,
    pub token: String,
}

#[derive(Debug, EsynDe, Default)]
pub struct MangadexConf {
    pub username: String,
    pub token: String,
}

#[derive(Debug, EsynDe, Default)]
pub struct BiliBiliConf {
    //pub follow_list: Vec,
    pub feed_list: Vec<i64>,
}

#[derive(Debug)]
pub struct Cli {
    pub cmd: String,
    pub any: Vec<OsString>,
}

// ========== IMPL App ==========
impl App {
    pub fn new() -> Res<Self> {
        let mut p = pico_args::Arguments::from_env();
        debug!("{:#?}", &p);

        let cli = Cli {
            cmd: p.subcommand()?.unwrap_or_else(|| Self::print_help()),
            any: p.finish(),
        };

        Ok(App {
            cli,
            config: Default::default(),
        })
    }

    pub fn main(self) -> Res<()> {
        use {
            crate::api::{anilist::Anilist, bilibili::BiliBili, fs::Fs, mangadex::MangaDex},
            crate::Query,
        };

        info!("{:#?}", &self);

        let cmd = &{ &self.cli.cmd.to_uppercase() };
        let (cmd, site) = cmd.split_once('.').unwrap();
        match Site::from_str(site)? {
            Site::Anilist => Anilist::new(self).main(cmd),
            Site::MangaDex => MangaDex::new(self).main(cmd),
            Site::BiliBili => BiliBili::new(self).main(cmd),
            Site::Fs => Fs::new(self).main(),
            _ => Self::print_help(),
        }
    }

    pub fn from_file(&mut self) -> Res<()> {
        let Some(dir) = dirs_next::config_dir()
        else { bail!("") };

        // ~/.config/emeta/config.rs
        let mut path = PathBuf::new();
        path.push(dir);
        path.push("emeta/config.rs");
        debug!("config path: {}", &path.display());

        let mut esyn = {
            let mut buf = "".to_string();
            File::open(path)?.read_to_string(&mut buf)?;

            Esyn::new(&buf)?
        };
        esyn.update::<Config>("main")?;

        let conf = esyn.get::<Config>("main", "config")?;
        self.config = conf;

        debug!("{:#?}", &self);

        Ok(())
    }

    pub fn print_help() -> ! {
        print!(
            r#"emeta: {VERSION}
RSUU    rsuuyu@gmail.com


USAGE:
  emeta <CMD> [OPTIONS|FLAGS] <INPUT>

ARGS:
  <INPUT>
          url OR number OR string

FLAGS:
  -h, --help
          Print help

COMMANDS:
  ls
  cat
  vi
          Edit
  dl
          Download
  news
  feed


OPTIONS:
  -u, --username
          username
"#
        );

        std::process::exit(0);
    }
}

impl App {
    pub fn filter_watch_status(&self) -> &WatchStatus {
        &self.config.setting.filter_watch_status
    }

    pub fn media_type(&self) -> &MediaType {
        &self.config.setting.media_type
    }

    pub fn any(&self) -> &Vec<OsString> {
        &self.cli.any
    }

    pub fn anilist(&self) -> &AnilistConf {
        &self.config.anilist_conf
    }

    pub fn mangadex(&self) -> &MangadexConf {
        &self.config.mangadex_conf
    }

    pub fn bilibili(&self) -> &BiliBiliConf {
        &self.config.bilibili_conf
    }
}
