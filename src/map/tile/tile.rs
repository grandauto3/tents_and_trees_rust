use std::cell::Cell;
use crate::{
    map::{
        tile::{
            grid_position::*,
            point::Point,
        }
    }
};

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
    coord: GridPosition,
    pub position: Cell<Point>,
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            tile_type: TileType::UNKNOWN,
            coord: GridPosition::default(),
            position: Cell::new(Point::default()),
        }
    }
}

impl Tile {
    pub fn new(tile_type: TileType, coord: GridPosition, position: Point) -> Self {
        Tile { tile_type, coord, position: Cell::new(position) }
    }

    pub fn get_tile_type(&self) -> &TileType {
        &self.tile_type
    }
}

