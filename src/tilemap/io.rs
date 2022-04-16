use std::{fs::File, path::Path};

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Tile {
    pub x: u32,
    pub y: u32,
    pub t: u16,
}

#[derive(Serialize, Deserialize)]

pub struct Tilemap {
    pub tiles: Vec<Tile>,
}

impl Tilemap {
    pub fn load<P: AsRef<Path>>(filename: P) -> Result<Self> {
        let file = File::open(filename)?;
        Ok(serde_yaml::from_reader(file)?)
    }
    pub fn save<P: AsRef<Path>>(&self, filename: P) -> Result<()> {
        let file = File::create(filename)?;
        Ok(serde_yaml::to_writer(file, &self)?)
    }
}
