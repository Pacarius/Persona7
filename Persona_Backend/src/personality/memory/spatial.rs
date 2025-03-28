use std::{collections::HashMap, fmt::Display, iter::Map};

use crate::world::{
    navigation::Navigator,
    world_map::{Region, Room, WorldMap},
};

#[derive(Debug)]
pub struct SpatialMemory {
    spatial_mem_tree: HashMap<String, HashMap<String, Vec<String>>>,
}
impl Default for SpatialMemory {
    fn default() -> Self {
        Self {
            spatial_mem_tree: HashMap::new(),
        }
    }
}
impl SpatialMemory {
    pub fn god(navigator: &Navigator) -> Self {
        let mut output = Self {
            spatial_mem_tree: HashMap::new(),
        };

        for region in &navigator.regions() {
            let region_name = region.name();
            if !output.spatial_mem_tree.contains_key(&region_name) {
                output
                    .spatial_mem_tree
                    .insert(region_name.clone(), HashMap::new());
            }

            let region_map = output.spatial_mem_tree.get_mut(&region_name).unwrap();

            for room in &region.rooms() {
                let room_name = room.name();
                if !region_map.contains_key(&room_name) {
                    region_map.insert(room_name.clone(), Vec::new());
                }

                let room_objects = region_map.get_mut(&room_name).unwrap();

                for object in navigator.objects() {
                    let (obj_region, obj_room) = (object.region(), object.room());
                    if obj_region == region_name && obj_room == room_name {
                        room_objects.push(object.name().clone());
                    }
                }
            }
        }

        output
    }
}
//REGION >> AREA
impl SpatialMemory {
    pub fn update(&mut self, region: Region, room: Room, objects: Vec<String>) {
        let region_name = region.name();
        let room_name = room.name();

        if !self.spatial_mem_tree.contains_key(&region_name) {
            self.spatial_mem_tree
                .insert(region_name.clone(), HashMap::new());
        }

        let region_map = self.spatial_mem_tree.get_mut(&region_name).unwrap();

        if !region_map.contains_key(&room_name) {
            region_map.insert(room_name.clone(), Vec::new());
        }

        let room_objects = region_map.get_mut(&room_name).unwrap();
        room_objects.extend(objects);
    }
}
impl Display for SpatialMemory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string_pretty(&self.spatial_mem_tree) {
            Ok(json) => write!(f, "{}", json),
            Err(err) => write!(f, "Error serializing to JSON: {}", err),
        }
    }
}
