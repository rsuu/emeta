use crate::{lib::tags, MyError, MyResult, SelfResult};
use miniserde::{json, Deserialize, Serialize};
use speedy::{Readable, Writable};
use std::{
    fs::{self, File},
    io::{self, Read, Write},
    path::Path,
};

pub trait TUpdateMeta {
    fn other(&mut self, data: &Option<impl AsRef<str>>);
    fn artist(&mut self, data: &Option<tags::TagArtist>);
    fn character(&mut self, data: &Option<tags::TagCharacter>);
    fn cosplayer(&mut self, data: &Option<tags::TagCosplayer>);
    fn female(&mut self, data: &Option<tags::TagFemale>);
    fn group(&mut self, data: &Option<tags::TagGroup>);
    fn language(&mut self, data: &Option<tags::TagLanguage>);
    fn male(&mut self, data: &Option<tags::TagMale>);
    fn mixed(&mut self, data: &Option<tags::TagMixed>);
    fn parody(&mut self, data: &Option<tags::TagParody>);
    fn reclass(&mut self, data: &Option<tags::TagReclass>);
    fn level(&mut self, data: &Option<tags::TagLevel>);
}

// RUSTFLAGS=-Zprint-type-sizes cargo run --release
#[repr(C)]
#[repr(align(8))]
#[derive(PartialEq, Eq, Debug, Readable, Writable, Serialize, Deserialize)]
pub struct MetaData {
    pub magick_number: u64,
    pub artist: Option<tags::TagArtist>,
    pub character: Option<tags::TagCharacter>,
    pub cosplayer: Option<tags::TagCosplayer>,
    pub female: Option<tags::TagFemale>,
    pub group: Option<tags::TagGroup>,
    pub language: Option<tags::TagLanguage>,
    pub male: Option<tags::TagMale>,
    pub mixed: Option<tags::TagMixed>,
    pub parody: Option<tags::TagParody>,
    pub reclass: Option<tags::TagReclass>,
    pub level: Option<tags::TagLevel>,
    pub other: Option<String>,
}

impl MetaData {
    pub fn new() -> Self {
        // u64::from_be_bytes(".rmgdata".as_bytes().try_into().unwrap())
        Self {
            magick_number: 3346857763922867297,
            artist: None,
            character: None,
            cosplayer: None,
            female: None,
            group: None,
            language: None,
            male: None,
            mixed: None,
            other: None,
            parody: None,
            reclass: None,
            level: None,
        }
    }

    pub fn display(&self) {
        println!("{:#?}", self);
    }

    pub fn from_pipe() -> SelfResult<Self> {
        let mut buffer = String::new();

        io::stdin().read_to_string(&mut buffer)?;

        Ok(Self::from_json(&buffer))
    }

    pub fn to_json(&self) -> String {
        json::to_string(self)
    }

    pub fn from_json(meta: &str) -> Self {
        json::from_str(meta).unwrap()
    }

    pub fn to_bytes(&self) -> SelfResult<Vec<u8>> {
        Ok(self.write_to_vec()?)
    }

    pub fn from_bytes<B>(bytes: &B) -> SelfResult<Self>
    where
        B: AsRef<[u8]>,
    {
        Ok(Self::read_from_buffer(bytes.as_ref())?)
    }

    pub fn write_to_file<P>(&self, filename: &P) -> MyResult
    where
        P: AsRef<Path> + ?Sized,
    {
        if filename.as_ref().is_file() {
        } else {
            let mut f = File::create(filename.as_ref())?;

            f.write_all(self.to_bytes()?.as_slice())?;
        }

        Ok(())
    }

    pub fn from_file<P>(filename: &P) -> SelfResult<Self>
    where
        P: AsRef<Path> + ?Sized,
    {
        let fmetadata = fs::metadata(filename.as_ref())?;
        let mut f = File::open(filename.as_ref())?;
        let mut buffer = vec![0; fmetadata.len() as usize];

        f.read(&mut buffer)?;

        Self::from_bytes(&buffer)
    }
}

impl Default for MetaData {
    fn default() -> Self {
        Self::new()
    }
}

impl TUpdateMeta for MetaData {
    fn other(&mut self, data: &Option<impl AsRef<str>>) {
        self.other = data.as_ref().map(|s| s.as_ref().to_string());
    }

    fn artist(&mut self, data: &Option<tags::TagArtist>) {
        self.artist = (*data).clone();
    }
    fn character(&mut self, data: &Option<tags::TagCharacter>) {
        self.character = (*data).clone();
    }
    fn cosplayer(&mut self, data: &Option<tags::TagCosplayer>) {
        self.cosplayer = (*data).clone();
    }
    fn female(&mut self, data: &Option<tags::TagFemale>) {
        self.female = (*data).clone();
    }
    fn group(&mut self, data: &Option<tags::TagGroup>) {
        self.group = (*data).clone();
    }
    fn language(&mut self, data: &Option<tags::TagLanguage>) {
        self.language = *data;
    }
    fn male(&mut self, data: &Option<tags::TagMale>) {
        self.male = (*data).clone();
    }
    fn reclass(&mut self, data: &Option<tags::TagReclass>) {
        self.reclass = *data;
    }
    fn parody(&mut self, data: &Option<tags::TagParody>) {
        self.parody = (*data).clone();
    }
    fn mixed(&mut self, data: &Option<tags::TagMixed>) {
        self.mixed = (*data).clone();
    }
    fn level(&mut self, data: &Option<tags::TagLevel>) {
        self.level = (*data).clone();
    }
}
