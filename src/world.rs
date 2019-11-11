//use std::collections::HashMap;

use slotmap::SlotMap;

use crate::entity::*;
use crate::map::*;
use crate::map_data::*;

pub struct World {
    pub map: Map,
    //pub entities: EntityList,
    pub entities: SlotMap<EntityId, Entity>,
    //pub factory: EntityFactory,
    pub data_store: Vec<EntityData>,
}

impl World {
    pub fn new(map_data: MapData) -> World {
        World {
            map: Map::new(map_data),
            entities: SlotMap::with_key(),
            //entities: EntityList::new(),
            //factory: EntityFactory::new(),
            data_store: Vec::new(),
        }
    }

    pub fn create_data(&mut self, name: &str, class: u8) -> &mut EntityData {
        let data_id = self.data_store.len() as EntityDataId;
        self.data_store.push(EntityData::new(data_id, String::from(name), class));
        &mut self.data_store[data_id]
    }

}