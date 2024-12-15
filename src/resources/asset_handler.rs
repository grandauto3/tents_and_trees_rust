use crate::map::tile::tile::TileType;

macro_rules! count_items {
    ($($item:expr),*) => {
        <[&str]>::len(&[$($item),*])
    };
}

macro_rules! create_const_asset_path_array {
    ($name:ident, {$($key:expr => $value:expr),* $(,)?}) => {
        const $name: [(TileType, &str); {count_items!($($value),*)}] = [
          $(($key ,concat!("./assets/",$value))),*
        ];
    };
}

create_const_asset_path_array!(ASSET_PATH_ARRAY, {
    TileType::Empty => "GrasTile.png",
    TileType::Tent =>"TentTile.png",
    TileType::Tree=>"TreeTile.png",
});

pub struct AssetHandler;

impl AssetHandler {
    pub fn get_path(tile_type: TileType) -> Option<&'static str> {
        let x = ASSET_PATH_ARRAY.iter().find(|e| e.0 == tile_type)?;
        Some(x.1)
    }

    pub fn get_path_vec() -> Vec<&'static str> {
        ASSET_PATH_ARRAY.iter().map(|e| e.1).collect()
    }
}
