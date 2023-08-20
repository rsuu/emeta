use esyn::EsynDe;
use std::{default, fmt, fmt::Display, str::FromStr};

#[derive(EsynDe, Debug, Clone, Copy, Default)]
pub enum WatchStatus {
    #[default]
    Unknown,
    Current,
    Planning,
    Completed,
    Dropped,
    Paused,
    Repeating,
}

impl Into<&'static str> for WatchStatus {
    fn into(self) -> &'static str {
        use WatchStatus::*;

        match self {
            Current => "CURRENT",
            Planning => "PLANNING",
            Completed => "COMPLETED",
            Dropped => "DROPPED",
            Paused => "PAUSED",
            Repeating => "REPEATING",
            Unknown => "UNKNOWN",
        }
    }
}

impl FromStr for WatchStatus {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use WatchStatus::*;

        Ok(match s.to_uppercase().as_str() {
            "CURRENT" => Current,
            "PLANNING" => Planning,
            "COMPLETED" => Completed,
            "DROPPED" => Dropped,
            "PAUSED" => Paused,
            "REPEATING" => Repeating,
            _ => Unknown,
        })
    }
}

impl Display for WatchStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value: &'static str = (*self).into();
        write!(f, "{}", value)
    }
}

#[derive(Debug, Default, Clone, Copy, EsynDe)]
pub enum Status {
    #[default]
    Current = 0,

    Cancelled,
    Completed,
    Repeating,
    Dropped,
    Finished,
    Hiatus,
    NotYetReleased,
    Paused,
    Planning,
    Releasing,
}

// ========== IMPL Status ==========
impl FromStr for Status {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_ascii_uppercase().as_str() {
            "CANCELLED" => Self::Cancelled,
            "COMPLETED" => Self::Completed,
            "CURRENT" => Self::Current,
            "DROPPED" => Self::Dropped,
            "FINISHED" => Self::Finished,
            "HIATUS" => Self::Hiatus,
            "NOTYETRELEASED" => Self::NotYetReleased,
            "PAUSED" => Self::Paused,
            "PLANNING" => Self::Planning,
            "RELEASING" => Self::Releasing,
            "REPEATING" => Self::Repeating,
            _ => unreachable!(),
        })
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", {
            match self {
                Self::Cancelled => "CANCELLED",
                Self::Completed => "COMPLETED",
                Self::Current => "CURRENT",
                Self::Dropped => "DROPPED",
                Self::Finished => "FINISHED",
                Self::Hiatus => "HIATUS",
                Self::NotYetReleased => "NOTYETRELEASED",
                Self::Paused => "PAUSED",
                Self::Planning => "PLANNING",
                Self::Releasing => "RELEASING",
                Self::Repeating => "REPEATING",
                _ => unreachable!(),
            }
        })
    }
}
