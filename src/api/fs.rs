use crate::*;

#[derive(Debug)]
pub struct Fs {
    app: App,
}

impl Fs {
    pub fn new(app: App) -> Self {
        Self { app }
    }

    pub fn main(&self) -> Res<()> {
        Ok(())
    }
}
