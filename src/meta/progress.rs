use std::fmt::Display;

#[derive(Debug)]
pub struct Progress {
    done: u16,
    total: Option<u16>,
}

impl Display for Progress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(total) = self.total {
            write!(f, "{}/{}", self.done, total)
        } else {
            write!(f, "{}", self.done)
        }
    }
}
