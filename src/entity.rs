//use std::collections::HashMap;
use slotmap::new_key_type;

use crate::position::*;

new_key_type! {
    pub struct EntityId;
}

pub type EntityDataId  = usize;

#[derive(Debug, Copy, Clone)]
pub struct Entity {
    //pub id: Option<EntityId>,
    pub data_id: EntityDataId,
    pub pos: Position,
    pub age: u16,
    pub lifetime: u16,
    pub energy: u16,
}

impl Entity {
    pub fn new(data: &EntityData, pos: Position) -> Entity {
        let lifetime = data.lifetime_min + rand::random::<u16>() % (data.lifetime_max - data.lifetime_min);

        Entity {
            //id: None,
            data_id: data.id,
            pos,
            age: 0,
            lifetime,
            energy: 0,
        }
    }
}

// pub struct EntityList {
//     entities: HashMap<EntityId, Entity>, // FIXME inefficient, most likely
// }

// impl EntityList {
//     pub fn new() -> EntityList {
//         EntityList {
//             entities: HashMap::new(),
//         }
//     }

//     pub fn add(&mut self, entity: Entity) {
//         self.entities.insert(entity.id, entity);
//     }

//     pub fn remove(&mut self, entity_id: EntityId) {
//         self.entities.remove(&entity_id);
//     }

//     pub fn for_each<F>(&self, mut action: F) where F: FnMut(&Entity) {
//         for (_entity_id, entity) in &self.entities {
//             action(entity);
//         }
//     }

//     pub fn for_each_mut<F>(&mut self, mut action: F) where F: FnMut(&mut Entity) {
//         for (_entity_id, entity) in &mut self.entities {
//             action(entity);
//         }
//     }

//     pub fn get(&self, id: EntityId) -> Option<&Entity> {
//         self.entities.get(&id)
//     }

//     pub fn get_mut(&mut self, id: EntityId) -> &mut Entity {
//         if let Some(entity) = self.entities.get_mut(&id) {
//             entity
//         }
//         else {
//             panic!("entity with id {} not found", id);
//         }
//     }

//     pub fn get_count(&self) -> usize {
//         return self.entities.len()
//     }
// }

#[derive(Debug)]
pub struct EntityData {
    pub id: EntityDataId,
    pub name: String,
    pub height_min: u8,
    pub height_max: u8,
    pub lifetime_min: u16,
    pub lifetime_max: u16,
    pub energy_age_min: u16,
    pub energy_age_max: u16,
    pub spawn_cost: u16,
    pub spawn_age_min: u16,
    pub spawn_age_max: u16,
    pub sprite: u16,
}

impl EntityData {
    pub fn new(id: EntityDataId, name: String) -> EntityData {
        EntityData {
            id,
            name,
            height_min: 0,
            height_max: 0,
            lifetime_min: 0,
            lifetime_max: 0,
            energy_age_min: 0,
            energy_age_max: 0,
            spawn_cost: 0,
            spawn_age_min: 0,
            spawn_age_max: 0,
            sprite: 0,
        }
    }

    pub fn set_height(&mut self, min: u8, max: u8) -> &mut EntityData {
        self.height_min = min;
        self.height_max = max;
        self
    }

    pub fn set_lifetime(&mut self, min: u16, max: u16) -> &mut EntityData {
        self.lifetime_min = min;
        self.lifetime_max = max;
        self
    }

    pub fn set_energy_age(&mut self, min: u16, max: u16) -> &mut EntityData {
        self.energy_age_min = min;
        self.energy_age_max = max;
        self
    }

    pub fn set_spawn(&mut self, cost: u16, age_min: u16, age_max: u16) -> &mut EntityData {
        self.spawn_cost = cost;
        self.spawn_age_min = age_min;
        self.spawn_age_max = age_max;
        self
    }

    pub fn set_sprite(&mut self, sprite: u16) -> &mut EntityData {
        self.sprite = sprite;
        self
    }
}

// pub struct EntityFactory {
//     entity_id_generator: EntityId,
// }

// impl EntityFactory {
//     pub fn new() -> EntityFactory {
//         EntityFactory {
//             entity_id_generator: 0,
//         }
//     }

//     pub fn create_entity(&mut self, data: &EntityData, pos: Position) -> Entity {
//         let entity_id = self.entity_id_generator;
//         self.entity_id_generator += 1;

//         Entity::new(entity_id, data, pos)
//     }
// }