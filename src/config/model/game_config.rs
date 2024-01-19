use serde::{Serialize, Deserialize};

const SIZE: usize = 12;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub map_config: MapConfig,
}

#[derive(Serialize, Deserialize)]
pub struct MapConfig {
    pub size: u32,
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
        MapConfig {
            size: SIZE as u32
        }
    }
}