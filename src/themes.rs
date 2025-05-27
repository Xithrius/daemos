use std::{fmt, str::FromStr};

use color_eyre::eyre::{Error, bail};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Copy, Default, PartialEq)]
pub enum AppTheme {
    #[default]
    Dark,
    Latte,
    Frappe,
    Macchiato,
    Mocha,
}

impl FromStr for AppTheme {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "dark" => Ok(Self::Dark),
            "latte" => Ok(Self::Latte),
            "frappe" => Ok(Self::Frappe),
            "macchiato" => Ok(Self::Macchiato),
            "mocha" => Ok(Self::Mocha),
            _ => bail!("Theme '{}' could not be deserialized", s),
        }
    }
}

impl fmt::Display for AppTheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            AppTheme::Dark => "Dark",
            AppTheme::Latte => "Latte",
            AppTheme::Frappe => "Frappe",
            AppTheme::Macchiato => "Macchiato",
            AppTheme::Mocha => "Mocha",
        };

        write!(f, "{}", label)
    }
}
