use enum_map::{
    enum_map,
    EnumMap,
};
use once_cell::unsync::Lazy;
use const_format::formatcp;

use crate::map::tile::tile::{TileType};


const ASSET_PATH: &str = "./assets";

pub const ASSET_PATH_MAP: Lazy<EnumMap<TileType, Option<&str>>> = Lazy::new(|| enum_map! {

    TileType::Empty => Some(formatcp!("{ASSET_PATH}/GrasTile.png")),
    TileType::Tent => Some(formatcp!("{ASSET_PATH}/TentTile.png")),
    TileType::Tree=> Some(formatcp!("{ASSET_PATH}/TreeTile.png")),
    _ => None
});

pub fn get_asset_paths_vec() -> Vec<&'static str> {
    ASSET_PATH_MAP.into_values().flatten().collect()
}