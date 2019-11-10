use slotmap::new_key_type;

use crate::position::{self, Position};

new_key_type! {
    pub struct EntityId;
}

pub type EntityDataId  = usize;

// FIXME use component instead!
pub const CLASS_PLANT: u8 = 0;
pub const CLASS_CREATURE: u8 = 1;

#[derive(Debug, Copy, Clone)]
pub struct Entity {
    pub data_id: EntityDataId,
    pub pos: Position,
    pub age: u16,
    pub lifetime: u16,
    pub height: u8,
    pub energy: u16,
}

impl Entity {
    pub fn new(data: &EntityData, pos: Position) -> Entity {
        let lifetime = data.lifetime_min + rand::random::<u16>() % (data.lifetime_max - data.lifetime_min);

        Entity {
            data_id: data.id,
            pos,
            age: 0,
            height: data.height,
            lifetime,
            energy: 0,
        }
    }
}

lazy_static::lazy_static! {
    pub static ref PATTERN_CLOSE: Vec<Position> = vec![
        position::LEFT,
        position::RIGHT,
        position::UP,
        position::DOWN,
    ];

    pub static ref PATTERN_CIRCLE: Vec<Position> = vec![
        position::LEFT,
        position::RIGHT,
        position::UP,
        position::DOWN,
        position::LEFT_UP,
        position::RIGHT_UP,
        position::LEFT_DOWN,
        position::RIGHT_DOWN,
        Position::new(-2, -1),
        Position::new(-2, 0),
        Position::new(-2, 1),
        Position::new(2, -1),
        Position::new(2, 0),
        Position::new(2, 1),
        Position::new(-1, -2),
        Position::new(0, -2),
        Position::new(1, -2),
        Position::new(-1, 2),
        Position::new(0, 2),
        Position::new(1, 2),
    ];
}

#[derive(Debug)]
pub struct EntityData {
    pub id: EntityDataId,
    pub name: String,
    pub class: u8, // FIXME use component instead!
    pub altitude_min: u8,
    pub altitude_max: u8,
    pub lifetime_min: u16,
    pub lifetime_max: u16,
    pub height: u8, // FIXME temp
    pub spawn_cost: u16,
    pub spawn_age_min: u16,
    pub spawn_age_max: u16,
    pub spawn_pattern: Option<&'static Vec<Position>>,
    pub sprite: u16,
}

impl EntityData {
    pub fn new(id: EntityDataId, name: String, class: u8) -> EntityData {
        EntityData {
            id,
            name,
            class,
            altitude_min: 0,
            altitude_max: 0,
            lifetime_min: 0,
            lifetime_max: 0,
            height: 0,
            spawn_cost: 0,
            spawn_age_min: 0,
            spawn_age_max: 0,
            spawn_pattern: None,
            sprite: 0,
        }
    }

    pub fn set_altitude(&mut self, min: u8, max: u8) -> &mut EntityData {
        self.altitude_min = min;
        self.altitude_max = max;
        self
    }

    pub fn set_lifetime(&mut self, min: u16, max: u16) -> &mut EntityData {
        self.lifetime_min = min;
        self.lifetime_max = max;
        self
    }

    pub fn set_height(&mut self, height: u8) -> &mut EntityData {
        self.height = height;
        self
    }

    pub fn set_spawn(&mut self, cost: u16, age_min: u16, age_max: u16, pattern: &'static Vec<Position>) -> &mut EntityData {
        self.spawn_cost = cost;
        self.spawn_age_min = age_min;
        self.spawn_age_max = age_max;
        self.spawn_pattern = Some(pattern);
        self
    }

    pub fn set_sprite(&mut self, sprite: u16) -> &mut EntityData {
        self.sprite = sprite;
        self
    }
}
