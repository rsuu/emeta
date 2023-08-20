use crate::Res;
use std::path::Path;

pub struct Editor {}
pub struct Term {}

impl Editor {
    pub fn open(&self, path: impl AsRef<Path>) -> Res<()> {
        let editor = std::env::var("VISUAL").unwrap_or(std::env::var("EDITOR")?);

        std::process::Command::new(editor)
            .arg(path.as_ref())
            .status()?;

        Ok(())
    }
}

impl Term {
    pub fn clear() {
        println!("\x1B[2J\x1B[1;1H");
    }
}
