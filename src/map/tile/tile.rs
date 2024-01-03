use crate::map::tile::point::*;

#[derive(Clone, Debug, PartialEq)]
pub enum TileType {
    UNKNOWN,
    EMPTY,
    TENT,
    TREE,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Tile {
    tile_type: TileType,
    coord: Point,
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            tile_type: TileType::UNKNOWN,
            coord: Point::default(),
        }
    }
}

impl Tile {
    pub fn new(tile_type: TileType, coord: Point) -> Self {
        Tile { tile_type, coord }
    }

    pub fn get_tile_type(&self) -> &TileType {
        &self.tile_type
    }
}

