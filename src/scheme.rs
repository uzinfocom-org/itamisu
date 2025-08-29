use crate::error::{ItamisuError, Result};
use rand::seq::IndexedRandom;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Food {
    name: String,
    shop: String,
}

#[derive(Default)]
pub struct ItamisuBuilder {
    data: String,
}

impl ItamisuBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn text<T>(self, text: T) -> Result<Self>
    where
        T: ToString,
    {
        Ok(Self {
            data: text.to_string(),
        })
    }

    pub fn read(self, path: PathBuf) -> Result<Self> {
        fs::read_to_string(path)
            .map_err(ItamisuError::CantAccessFile)
            .and_then(|data| Ok(Self { data }))
    }

    pub fn build(self) -> Result<Itamisu> {
        toml::from_str(&self.data).map_err(ItamisuError::CantParseFile)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Itamisu {
    option: Vec<Food>,
}

impl Itamisu {
    pub fn builder() -> ItamisuBuilder {
        ItamisuBuilder::new()
    }

    pub fn random(&self, rep: u8) -> Result<Itamisu> {
        Ok(Self {
            option: self
                .option
                .choose_multiple(&mut rand::rng(), rep.into())
                .map(|i| i.to_owned())
                .collect(),
        })
    }

    pub fn show(self) -> Result<()> {
        self.option.iter().for_each(|f| println!("{}", f.name));

        Ok(())
    }

    pub fn write(self, mut path: PathBuf) -> Result<()> {
        if !path.ends_with(".toml") {
            path.push("foods.toml");
        }

        fs::write(
            path,
            toml::to_string_pretty(&self).map_err(ItamisuError::CantConvertFile)?,
        )
        .map_err(ItamisuError::CantAccessFile)
    }
}
