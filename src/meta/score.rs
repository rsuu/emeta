use esyn::EsynDe;

/// AniList score formats.
#[derive(Clone, Default, Copy, Debug, EsynDe)]
pub enum Score {
    #[default]
    /// Range between 0 - 100.
    Point100,
    /// Range between 0.0 - 10.0.
    Point10Decimal,
    /// Range between 0 - 10.
    Point10,
    /// Range between 0 - 5.
    Point5,
    /// Range between 0 - 100. This variant is unique in that it is
    /// represented by an ASCII-style face. Value ranges for each face
    /// are shown below:
    ///
    /// | Range    | Face |
    /// | -------- | ---- |
    /// | 0 - 33   | :(   |
    /// | 34 - 66  | :\|  |
    /// | 67 - 100 | :)   |
    Point3,
}

impl Score {
    fn points_value(&self, score: &str) -> Option<u8> {
        use Score::*;

        let raw_score = match &self {
            Point100 => score.parse().ok()?,
            Point10Decimal => {
                let score = score.parse::<f32>().ok()?;
                (score * 10.0).round() as u8
            }
            Point10 => {
                let score = score.parse::<u8>().ok()?;
                score.saturating_mul(10)
            }
            Point5 => {
                let score = score.parse::<u8>().ok()?;
                score.saturating_mul(20)
            }
            Point3 => match score {
                ":(" => 33,
                ":|" => 50, // When set to 66, AniList interprets this as the ":)" rating
                ":)" => 100,
                _ => return None,
            },
        };

        Some(raw_score.min(100))
    }
}
