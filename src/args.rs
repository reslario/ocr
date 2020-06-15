use {
    std::fmt,
    structopt::StructOpt
};

#[derive(StructOpt)]
pub struct Args {
    /// Path to the file to scan
    pub file: std::path::PathBuf,
    /// Language of the text
    #[structopt(long, short, default_value)]
    pub lang: Language
}

#[derive(serde::Serialize)]
pub struct Language(String);

impl Default for Language {
    fn default() -> Self {
        Language("eng".into())
    }
}

impl std::str::FromStr for Language {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Language(s.into()))
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

pub fn args() -> Args {
    StructOpt::from_args()
}