use crate::entity::EntityId;
use crate::map_data::MapData;
use crate::position::Position;

pub struct Tile {
    pub pos: Position,
    pub height: u8,
    pub entity: Option<EntityId>,
    pub entity_height: u8,
    pub booked: bool,
}

impl Tile {
    fn new(x: i16, y: i16, height: u8) -> Tile {
        Tile {
            pos: Position::new(x, y),
            height: height,
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
    tiles: Vec<Tile>,
    width: usize,
}

impl Map {
    pub fn new(data: MapData) -> Map {
        let mut tiles: Vec<Tile> = Vec::new();
        for iy in 0..data.get_height() {
            for ix in 0..data.get_width() {
                let tile = Tile::new(ix as i16, iy as i16, data.get_tile(ix, iy));
                tiles.push(tile);
            }
        }

        Map {
            tiles,
            width: data.get_width(),
        }
    }

    pub fn get(&self, pos: Position) -> Option<&Tile> {
        self.tiles.get(pos.y as usize * self.width + pos.x as usize)
    }

    pub fn get_mut(&mut self, pos: Position) -> Option<&mut Tile> {
        self.tiles.get_mut(pos.y as usize * self.width + pos.x as usize)
    }

    pub fn for_each<F>(&self, mut action: F) where F: FnMut(&Tile) {
        for tile in &self.tiles {
            action(tile);
        }
    }
}