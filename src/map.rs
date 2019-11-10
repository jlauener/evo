use std::collections::HashMap;

use crate::entity::*;
use crate::position::*;

pub struct Tile {
    pub pos: Position,
    pub height: u8,
    pub entity: Option<EntityId>,
    pub entity_height: u8,
    pub booked: bool,
}

impl Tile {
    fn new(x: i16, y: i16) -> Tile {
        Tile {
            pos: Position::new(x, y),
            height: 0,
            entity: None,
            entity_height: 0,
            booked: false,
        }
    }

    pub fn is_free(&self) -> bool {
        !self.booked && self.entity.is_none()
    }
}

pub struct Map {
    tiles: HashMap<(i16, i16), Tile>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Map {
        let mut tiles = HashMap::new();
        for iy in 0..height as i16 {
            for ix in 0..width as i16 {
                tiles.insert((ix, iy), Tile::new(ix, iy));
            }
        }
        Map { tiles }
    }

    pub fn get(&self, pos: Position) -> Option<&Tile> {
        self.tiles.get(&(pos.x, pos.y))
    }

    pub fn get_mut(&mut self, pos: Position) -> Option<&mut Tile> {
        self.tiles.get_mut(&(pos.x, pos.y))
    }

    pub fn for_each<F>(&self, mut action: F) where F: FnMut(&Tile) {
        for (_, tile) in &self.tiles {
            action(tile);
        }
    }
}