use std::{collections::HashMap, fmt::Display, iter::Map};

use crate::world::world_map::{Region, Room};

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
//REGION >> AREA
impl SpatialMemory {
    pub fn update(&mut self, region: Region, room: Room, objects: Vec<String>) {
        let region_name = region.name();
        let room_name = room.name();

        // Check if the region exists, if not insert it
        if !self.spatial_mem_tree.contains_key(&region_name) {
            self.spatial_mem_tree
                .insert(region_name.clone(), HashMap::new());
        }

        // Get the region's map
        let region_map = self.spatial_mem_tree.get_mut(&region_name).unwrap();

        // Check if the room exists within the region, if not insert it
        if !region_map.contains_key(&room_name) {
            region_map.insert(room_name.clone(), Vec::new());
        }

        // Update the list of objects for the specified room
        let room_objects = region_map.get_mut(&room_name).unwrap();
        room_objects.extend(objects);
    }
}
impl Display for SpatialMemory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (region, rooms) in &self.spatial_mem_tree {
            writeln!(f, "Region: {}", region)?;
            for (room, objects) in rooms {
                writeln!(f, "\tRoom: {}", room)?;
                for object in objects {
                    writeln!(f, "\t\tObject: {}", object)?;
                }
            }
        }
        Ok(())
    }
}
