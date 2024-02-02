use std::iter::Map;
use serde::{Serialize, Deserialize};
use toml::{
    Table
};

const SIZE: u32 = 12;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub map_config: MapConfig,
}

#[derive(Serialize, Deserialize)]
pub struct MapConfig {
    pub size: Table,
}

impl MapConfig {
    pub fn get_size(&self) -> (usize, usize) {
        (
            self.size.get("x").unwrap().as_integer().unwrap() as usize,
            self.size.get("y").unwrap().as_integer().unwrap() as usize,
        )
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            map_config: MapConfig::default()
        }
    }
}

impl Default for MapConfig {
    fn default() -> Self {
        let mut default_table = Table::new();
        default_table.insert("x".to_owned(), SIZE.into());
        default_table.insert("y".to_owned(), SIZE.into());

        MapConfig {
            size: default_table,
        }
    }
}