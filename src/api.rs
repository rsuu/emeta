/// anime/manga/novel
pub mod anilist;
/// video
pub mod bilibili;
/// image
pub mod danbooru;
/// game
pub mod dlsite;
/// metadata
pub mod fs;
/// movie/tv
pub mod imdb;
/// manga
pub mod mangadex;
/// novel
pub mod novelupdate;
/// video
pub mod youtube;

use crate::*;

// Query  <- Struct
// Query  -> JSON
// Struct <- JSON
pub trait Query {
    fn main(&self, v: &str) -> Res<()> {
        match v {
            "LS" => self.ls(),
            "FD" => self.fd(),
            "VI" => self.vi(),
            "CAT" => self.cat(),
            "FEED" => self.feed(),
            "NEWS" => self.news(),
            "CP" => self.cp(),
            "SET" => self.set(),

            _ => bail!("Unknown Command"),
        }
    }

    fn ls(&self) -> Res<()> {
        Ok(())
    }

    fn cat(&self) -> Res<()> {
        Ok(())
    }

    fn feed(&self) -> Res<()> {
        Ok(())
    }

    fn news(&self) -> Res<()> {
        Ok(())
    }

    fn rm(&self) -> Res<()> {
        Ok(())
    }

    fn cp(&self) -> Res<()> {
        Ok(())
    }

    fn mv(&self) -> Res<()> {
        Ok(())
    }

    fn fd(&self) -> Res<()> {
        Ok(())
    }

    fn vi(&self) -> Res<()> {
        Ok(())
    }

    fn dl(&self) -> Res<()> {
        Ok(())
    }

    fn set(&self) -> Res<()> {
        Ok(())
    }
}
