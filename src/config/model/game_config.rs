use serde::{Serialize, Deserialize};
use toml::{
    Table
};

const SIZE: u32 = 12;
const SIZE_OF_TILE: f32 = 50f32;

#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    pub map_config: MapConfig,
}

#[derive(Serialize, Deserialize)]
pub struct MapConfig {
    size: Table,
    size_of_tile: f32,
}

impl MapConfig {
    pub fn get_map_size(&self) -> (usize, usize) {
        (
            self.size.get("x").unwrap().as_integer().unwrap() as usize,
            self.size.get("y").unwrap().as_integer().unwrap() as usize,
        )
    }
    pub fn set_size(&mut self, new: (usize, usize)) {
        *self.size.get_mut("x").unwrap() = toml::Value::Integer(new.0 as i64);
        *self.size.get_mut("y").unwrap() = toml::Value::Integer(new.1 as i64);
    }

    pub fn get_tile_size(&self) -> (f32, f32) {
        (self.size_of_tile, self.size_of_tile)
    }
}

impl Default for MapConfig {
    fn default() -> Self {
        let mut default_table = Table::new();
        default_table.insert("x".to_owned(), SIZE.into());
        default_table.insert("y".to_owned(), SIZE.into());

        MapConfig {
            size: default_table,
            size_of_tile: SIZE_OF_TILE,
        }
    }
}