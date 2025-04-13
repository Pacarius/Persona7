use std::collections::{HashMap, HashSet, VecDeque};

use super::{
    character::Character,
    utils::{MapObject, Region, Room},
    world_map::{Coordinates, WorldMap},
};

pub struct Navigator {
    regions: Vec<Region>,
    rooms: Vec<Room>,
    colliders: Vec<Vec<Option<String>>>,
    objects: Vec<MapObject>,
    size: Coordinates,
}
impl Navigator {
    pub fn new(map: &WorldMap) -> Self {
        let regions = map.region_slice();
        let rooms = map.room_slice();
        let colliders = map.collider_slice();
        let objects = map.object_slice();
        let size = map.size();
        Navigator {
            regions,
            rooms,
            colliders,
            objects,
            size,
        }
    }
    fn colliders(&self) -> &Vec<Vec<Option<String>>> {
        if self.colliders.len() < 1 {
            panic!(
                "{}",
                ("Colliders list is empty. Did you remember to update them?")
            );
        }
        &self.colliders
    }
    pub fn objects(&self) -> Vec<&MapObject> {
        self.objects.iter().map(|f| f).collect()
    }
    pub fn regions(&self) -> Vec<&Region> {
        self.regions.iter().map(|r| r).collect()
    }
    pub fn size(&self) -> &Coordinates {
        &self.size
    }
    // pub fn update_colliders(&mut self, map: &WorldMap) {
    //     self.colliders = map.collider_slice();
    // }
    pub fn get_path(&self, from: Coordinates, to: Coordinates) -> Option<VecDeque<Coordinates>> {
        // if self.colliders.len() < 1 {Err((""));}
        // println!("Trying to find path from {} to {}", from, to);
        let colliders = self.colliders();
        let (start_x, start_y) = (from.0, from.1);
        let (goal_x, goal_y) = (to.0, to.1);

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut came_from = HashMap::new();

        queue.push_back((start_x, start_y));
        visited.insert((start_x, start_y));

        while let Some((x, y)) = queue.pop_front() {
            if (x, y) == (goal_x, goal_y) {
                let mut path = VecDeque::new();
                let mut current = (goal_x, goal_y);

                while current != (start_x, start_y) {
                    path.push_back(Coordinates(current.0, current.1));
                    current = came_from[&current];
                }

                path.push_back(Coordinates(start_x, start_y));
                return Some(path.iter().rev().cloned().collect());
            }

            for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let new_x = (x as isize + dx) as usize;
                let new_y = (y as isize + dy) as usize;

                if new_x < self.size.0
                    && new_y < self.size.1
                    && colliders[new_x][new_y].is_none()
                    && !visited.contains(&(new_x, new_y))
                {
                    queue.push_back((new_x, new_y));
                    visited.insert((new_x, new_y));
                    came_from.insert((new_x, new_y), (x, y));
                }
            }
        }
        return None;
    }
    pub fn get_pos_room(&self, location: (String, String)) -> Option<Coordinates> {
        let (region_name, room_name) = location;
        let regions = &self.regions;
        let colliders = self.colliders();
        let region = regions.iter().find(|r| r.name() == region_name)?;

        let rooms = region.rooms();
        let room = rooms.iter().find(|r| r.name() == room_name)?;

        for x in room.position().0..(room.position().0 + room.size().0) {
            for y in room.position().1..(room.position().1 + room.size().1) {
                let coord = Coordinates(x, y);
                if colliders[x][y].is_none() {
                    return Some(coord);
                }
            }
        }
        None
    }
    pub fn get_position_info(&self, position: &Coordinates) -> Option<(String, String)> {
        let regions = &self.regions;
        return regions.iter().find_map(|region| {
            if position.0 >= region.position().0
                && position.0 < region.position().0 + region.size().0
                && position.1 >= region.position().1
                && position.1 < region.position().1 + region.size().1
            {
                region.rooms().iter().find_map(|room| {
                    if position.0 >= room.position().0
                        && position.0 < room.position().0 + room.size().0
                        && position.1 >= room.position().1
                        && position.1 < room.position().1 + room.size().1
                    {
                        Some((region.name(), room.name()))
                    } else {
                        None
                    }
                })
            } else {
                None
            }
        });
        None
    }
    // pub fn get_visible_objects(
    //     &self,
    //     character: &Character,
    // ) -> HashMap<String, (bool, Vec<Coordinates>)> {
    //     let (source_name, source_pos, v_range): (&String, &Coordinates, &i64) =
    //         (character.name(), character.position(), character.v_range());
    //     let mut visible_objects: HashMap<String, (bool, Vec<Coordinates>)> = HashMap::new();

    //     for object in &self.objects {
    //         let object_pos = object.position();
    //         let distance = ((object_pos.0 as i64 - source_pos.0 as i64).pow(2)
    //             + (object_pos.1 as i64 - source_pos.1 as i64).pow(2))
    //         .isqrt() as i64;
    //         // println!("{}", distance);
    //         if distance <= *v_range {
    //             let mut obstructed = false;
    //             let (x0, y0) = (source_pos.0 as i64, source_pos.1 as i64);
    //             let (x1, y1) = (object_pos.0 as i64, object_pos.1 as i64);

    //             // Bresenham's line algorithm to check for walls
    //             let dx = (x1 - x0).abs();
    //             let dy = -(y1 - y0).abs();
    //             let mut err = dx + dy;
    //             let mut x = x0;
    //             let mut y = y0;

    //             let sx = if x0 < x1 { 1 } else { -1 };
    //             let sy = if y0 < y1 { 1 } else { -1 };

    //             while x != x1 || y != y1 {
    //                 if let Some(collider) = &self.colliders()[x as usize][y as usize] {
    //                     if collider == "Wall" {
    //                         obstructed = true;
    //                         break;
    //                     }
    //                 }

    //                 let e2 = 2 * err;
    //                 if e2 >= dy {
    //                     err += dy;
    //                     x += sx;
    //                 }
    //                 if e2 <= dx {
    //                     err += dx;
    //                     y += sy;
    //                 }
    //             }

    //             if !obstructed && *object.owner() == None {
    //                 visible_objects
    //                     .entry(object.name().clone())
    //                     .or_insert_with(|| (object.collision(), Vec::new()))
    //                     .1
    //                     .push(object.position().clone());
    //             }
    //         }
    //     }

    //     visible_objects
    // }
    pub fn get_visible_objects(
        &self,
        character: &Character,
    ) -> HashMap<String, (bool, Vec<Coordinates>)> {
        let (source_name, source_pos, v_range): (&String, &Coordinates, &i64) =
            (character.name(), character.position(), character.v_range());
        let mut visible_objects: HashMap<String, (bool, Vec<Coordinates>)> = HashMap::new();

        // Get the room the character is currently in
        if let Some((_, room_name)) = self.get_position_info(source_pos) {
            for object in &self.objects {
                // Check if the object is in the same room
                if let Some((_, object_room_name)) = self.get_position_info(object.position()) {
                    if object_room_name == room_name {
                        // Calculate the distance between the character and the object
                        let distance = ((object.position().0 as i64 - source_pos.0 as i64).pow(2)
                            + (object.position().1 as i64 - source_pos.1 as i64).pow(2))
                        .isqrt() as i64;

                        // If the object is within the character's visibility range, add it
                        if distance <= *v_range {
                            visible_objects
                                .entry(object.name().clone())
                                .or_insert_with(|| (object.collision(), Vec::new()))
                                .1
                                .push(object.position().clone());
                        }
                    }
                }
            }
        }

        visible_objects
    }
    //  pub fn get_path(&self, name: String, target: Coordinates) -> Option<VecDeque<Coordinates>> {
    //         let start = &self.get_character(name).position();
    //         let goal = &target;

    //         let (start_x, start_y) = (start.0, start.1);
    //         let (goal_x, goal_y) = (goal.0, goal.1);

    //         let mut queue = VecDeque::new();
    //         let mut visited = HashSet::new();
    //         let mut came_from = HashMap::new();

    //         queue.push_back((start_x, start_y));
    //         visited.insert((start_x, start_y));

    //         while let Some((x, y)) = queue.pop_front() {
    //             if (x, y) == (goal_x, goal_y) {
    //                 let mut path = VecDeque::new();
    //                 let mut current = (goal_x, goal_y);

    //                 while current != (start_x, start_y) {
    //                     path.push_back(Coordinates(current.0, current.1));
    //                     current = came_from[&current];
    //                 }

    //                 path.push_back(Coordinates(start_x, start_y));
    //                 return Some(path.iter().rev().cloned().collect());
    //             }

    //             for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
    //                 let new_x = (x as isize + dx) as usize;
    //                 let new_y = (y as isize + dy) as usize;

    //                 if new_x < self.size.0
    //                     && new_y < self.size.1
    //                     && self.colliders[new_x][new_y].is_none()
    //                     && !visited.contains(&(new_x, new_y))
    //                 {
    //                     queue.push_back((new_x, new_y));
    //                     visited.insert((new_x, new_y));
    //                     came_from.insert((new_x, new_y), (x, y));
    //                 }
    //             }
    //         }

    //         None
    //     }
}
