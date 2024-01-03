mod map;

use std::hash::Hash;
use rand;
use rand::Rng;
use crate::map::{tile::tile::*, map::*};


fn main() {
    const SIZE: usize = 12;

    let mut map = create_map(SIZE);

    for row_index in 0..map.row_len() {
        for col_index in 0..map.column_len() {
            let neighbours = get_neighbours((row_index, col_index), &map);
            println!("Row: {row_index}, Column: {col_index}, Tile: {:#?}\n{:#?}", map.get(row_index, col_index).unwrap().get_tile_type(), neighbours);
        }
    }

    draw_map(&map);

    let count_tree = map.elements_row_major_iter().filter(move |x| { *x.get_tile_type() == TileType::TREE }).count();

    println!("Tree spaces are: {count_tree}");
}

