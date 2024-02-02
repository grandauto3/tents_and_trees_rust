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