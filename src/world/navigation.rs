use std::collections::{HashMap, HashSet, VecDeque};

use super::world_map::{Coordinates, Region, Room};

pub struct Navigator {
    regions: Option<Vec<Region>>,
    rooms: Option<Vec<Room>>,
    colliders: Option<Vec<Vec<Option<String>>>>,
    size: Coordinates,
}
impl Navigator {
    pub fn new(
        regions: Option<Vec<Region>>,
        rooms: Option<Vec<Room>>,
        colliders: Option<Vec<Vec<Option<String>>>>,
        size: Coordinates,
    ) -> Self {
        Navigator {
            regions,
            rooms,
            colliders,
            size,
        }
    }
    pub fn get_path(&self, from: Coordinates, to: Coordinates) -> Option<VecDeque<Coordinates>> {
        if let Some(colliders) = (&self.colliders) {
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
        None
    }
    pub fn get_pos_room(&self, location: (String, String)) -> Option<Coordinates> {
        let (region_name, room_name) = location;
        if let Some(regions) = &self.regions {
            if let Some(colliders) = &self.colliders {
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
                return None;
            }
            return None;
        }
        None
    }
    pub fn get_position_info(&self, position: &Coordinates) -> Option<(String, String)> {
        if let Some(regions) = &self.regions {
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
        }
        None
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
