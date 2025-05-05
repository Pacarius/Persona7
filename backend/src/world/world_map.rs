use core::time;
use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::future;
use std::iter::Map;
use std::marker::PhantomData;

use std::{
    any::Any,
    collections::HashMap,
    fmt::{Debug, Display},
    hash::Hash,
};

use futures::future::join_all;
// use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
// use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use tokio::join;

use crate::misc::ollama::ollama::Ollama;
use crate::misc::time::{Date, DateTime, Time};
use crate::personality::action::ActionEntry;

use super::character::Character;
use super::navigation::Navigator;
use super::utils::{MapObject, Region, Room};

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
//AT SOME POINT THE X,Y COORDINATES OF EVERYTHING FLIPPED KILL ME
pub struct Coordinates(pub usize, pub usize);
impl Display for Coordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}
// pub struct Character(Identity, String);
pub struct WorldMap {
    size: Coordinates,
    regions: Vec<Region>,
    objects: Vec<MapObject>,
    characters: Vec<Character>,
    walls: Vec<Coordinates>,
    pub colliders: Vec<Vec<Option<String>>>,
    objects_buffer: Vec<String>,
}
impl WorldMap {
    pub fn new(size: Coordinates) -> Self {
        let (x, y) = (size.0, size.1);
        Self {
            size,
            regions: Vec::new(),
            objects: Vec::new(),
            characters: Vec::new(),
            walls: Vec::new(),
            colliders: vec![vec![None; x]; y],
            objects_buffer: vec![], // room_region_map: HashMap::new()
        }
    }
    pub fn add_region(&mut self, region: Region) {
        self.regions.push(region);
    }
    pub fn add_walls(&mut self) {
        let mut holes = vec![];
        for region in &self.regions {
            for room in &region.rooms() {
                if room.is_walled() {
                    let (x_top, y_top) = (room.position().0, room.position().1);
                    let (x_size, y_size) = (room.size().0, room.size().1);
                    holes.extend(room.holes());
                    // Add top and bottom walls
                    let mut walls = vec![];
                    for x in x_top..(x_top + x_size) {
                        walls.push(Coordinates(x, y_top));
                        walls.push(Coordinates(x, y_top + y_size - 1));
                    }
                    // Add left and right walls
                    for y in y_top..(y_top + y_size) {
                        walls.push(Coordinates(x_top, y));
                        walls.push(Coordinates(x_top + x_size - 1, y));
                    }
                    self.walls.extend(walls);
                }
            }
            holes.dedup();
            self.walls.retain(|w| !holes.contains(&w));
        }
    }
    fn get_position_info(&self, position: &Coordinates) -> Option<(String, String)> {
        // if let Some(regions) = &self.regions {
        return self.regions.iter().find_map(|region| {
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
    pub fn calculate_colliders(&mut self) {
        self.colliders = vec![vec![None; self.size.0]; self.size.1];
        for w in &self.walls {
            self.colliders[w.0][w.1] = Some("Wall".to_string());
        }
        let position_infos: Vec<(usize, usize, Option<(String, String)>)> = self
            .objects
            .iter()
            .map(|o| {
                let Coordinates(x, y) = o.position();
                let position_info = self.get_position_info(&Coordinates(*x, *y));
                (*x, *y, position_info)
            })
            .collect::<Vec<(usize, usize, Option<(String, String)>)>>();

        for (i, o) in self.objects.iter_mut().enumerate() {
            if !o.collision() {
                continue;
            }
            let Coordinates(x, y) = o.position();
            let vertical = o.horizontal() as usize;
            let horizontal = o.vertical() as usize;

            for i in 0..vertical {
                for j in 0..horizontal {
                    let (x_pos, y_pos) = (x + i, y + j);
                    if self.colliders[x_pos][y_pos].is_none() {
                        self.colliders[x_pos][y_pos] = Some(o.name().clone());
                    } else {
                        println!(
                            "Collider already exists at ({}, {}): {}",
                            x_pos,
                            y_pos,
                            self.colliders[x_pos][y_pos].as_ref().unwrap()
                        );
                    }
                }
            }
            if let Some((region, room)) = &position_infos[i].2 {
                o.late(region.clone(), room.clone());
            }
        }
        self.characters.iter().for_each(|c| {
            let Coordinates(x, y) = c.position();
            if self.colliders[*x][*y].is_none() {
                self.colliders[*x][*y] = Some(c.name().clone());
            } else {
                println!(
                    "Collider already exists at ({}, {}): {}",
                    x,
                    y,
                    self.colliders[*x][*y].as_ref().unwrap()
                );
            }
        });
    }
    // pub fn move_characters(&mut self) -> Vec<(Coordinates, Coordinates)> {
    //     let mut moved_positions = Vec::new();

    //     self.characters.iter_mut().for_each(|c| {
    //         let pos = c.position().clone();
    //         if let Some(pos) = c._move() {
    //             if self.colliders[pos.0 .0][pos.0 .1] == Some(c.name().clone()) {
    //                 self.colliders[pos.0 .0][pos.0 .1] = None;
    //                 self.colliders[pos.1 .0][pos.1 .1] = Some(c.name().clone());
    //             }
    //             moved_positions.push(pos);
    //         }
    //     });
    //     moved_positions
    // }
    pub fn add_object(&mut self, object: MapObject) {
        self.objects.push(object);
    }
    pub fn add_character(&mut self, character: Character) {
        if self.characters.iter().any(|f| f.name() == character.name()) {
            println!("{} already exists.", character.name());
        }
        self.characters.push(character);
    }
    fn regions_as_chars(&self) -> Vec<Vec<char>> {
        let mut region_map = vec![vec![' '; self.size.0]; self.size.1];

        for region in &self.regions {
            let Coordinates(x_top, y_top) = region.position().clone();
            let Coordinates(x_size, y_size) = region.size().clone();

            for x in x_top..(x_top + x_size) {
                for y in y_top..(y_top + y_size) {
                    if x < self.size.0 && y < self.size.1 {
                        region_map[y][x] = region.name().chars().next().unwrap_or(' ');
                    }
                }
            }
        }

        region_map
    }

    fn as_chars(&self) -> Vec<Vec<char>> {
        let mut collider_map = vec![vec![' '; self.size.0]; self.size.1];

        for (y, row) in self.colliders.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if let Some(ref obj) = cell {
                    collider_map[x][y] = obj.chars().next().unwrap();
                }
            }
        }

        collider_map
    }
    pub fn get_character(&self, name: String) -> &Character {
        self.characters
            .iter()
            .filter(|n| n.name().eq(&name))
            .nth(0)
            .unwrap()
    }
    pub fn get_character_mut(&mut self, name: String) -> &mut Character {
        self.characters
            .iter_mut()
            .filter(|n| n.name().eq(&name))
            .nth(0)
            .unwrap()
    }
    pub fn get_characters_slice(&self) -> Vec<Character> {
        self.characters.iter().cloned().collect()
    }
    pub fn get_characters(&self) -> Vec<&Character> {
        self.characters.iter().collect()
    }
    pub fn get_characters_mut(&mut self) -> Vec<&mut Character> {
        self.characters.iter_mut().collect()
    }
    // pub fn get_character(&self, name: String) -> &Character{
    //     self.characters.iter().filter(|f| f.name == name).nth(0).unwrap()
    // }
    // pub fn get_path(&self, name: String, target: Coordinates) -> Option<VecDeque<Coordinates>> {
    //     let start = &self.get_character(name).position();
    //     let goal = &target;

    //     let (start_x, start_y) = (start.0, start.1);
    //     let (goal_x, goal_y) = (goal.0, goal.1);

    //     let mut queue = VecDeque::new();
    //     let mut visited = HashSet::new();
    //     let mut came_from = HashMap::new();

    //     queue.push_back((start_x, start_y));
    //     visited.insert((start_x, start_y));

    //     while let Some((x, y)) = queue.pop_front() {
    //         if (x, y) == (goal_x, goal_y) {
    //             let mut path = VecDeque::new();
    //             let mut current = (goal_x, goal_y);

    //             while current != (start_x, start_y) {
    //                 path.push_back(Coordinates(current.0, current.1));
    //                 current = came_from[&current];
    //             }

    //             path.push_back(Coordinates(start_x, start_y));
    //             return Some(path.iter().rev().cloned().collect());
    //         }

    //         for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
    //             let new_x = (x as isize + dx) as usize;
    //             let new_y = (y as isize + dy) as usize;

    //             if new_x < self.size.0
    //                 && new_y < self.size.1
    //                 && self.colliders[new_x][new_y].is_none()
    //                 && !visited.contains(&(new_x, new_y))
    //             {
    //                 queue.push_back((new_x, new_y));
    //                 visited.insert((new_x, new_y));
    //                 came_from.insert((new_x, new_y), (x, y));
    //             }
    //         }
    //     }

    //     None
    // }
    // pub fn get_path_visual(&self, name: String) -> String {
    //     let character = self.get_character(name.clone());
    //     let path = match character.path() {
    //         Some(p) => p,
    //         None => return "No path available.".to_string(),
    //     };

    //     let mut map = self.as_chars();

    //     for Coordinates(x, y) in path {
    //         if x < &self.size.0 && y < &self.size.1 {
    //             map[*y][*x] = '*';
    //         }
    //     }

    //     map.iter()
    //         .map(|row| row.iter().map(|&c| format!("{} ", c)).collect::<String>())
    //         .collect::<Vec<String>>()
    //         .join("\n")
    // }
    // pub fn set_path(&mut self, name: String, position: Coordinates) {
    //     match self.get_path(name.clone(), position) {
    //         Some(o) => self.get_character_mut(name).set_path(o),
    //         None => println!("No paths available."),
    //     }
    //     // self.get_character_mut(name).set_path(self.get_path(name, position));
    // }
    // pub fn get_pos_room(&self, location: (String, String)) -> Option<Coordinates> {
    //     let (region_name, room_name) = location;

    //     let region = self.regions.iter().find(|r| r.name == region_name)?;

    //     let room = region.rooms.iter().find(|r| r.name == room_name)?;

    //     for x in room.position.0..(room.position.0 + room.size.0) {
    //         for y in room.position.1..(room.position.1 + room.size.1) {
    //             let coord = Coordinates(x, y);
    //             if self.colliders[x][y].is_none() {
    //                 return Some(coord);
    //             }
    //         }
    //     }

    //     None
    // }
    // pub fn set_path_character(
    //     &self,
    //     character: &mut Character,
    //     location: (String, String),
    // ) -> Result<VecDeque<Coordinates>, Box<dyn Error>> {
    //     if let Some(pos) = self.get_pos_room(location) {
    //         if let Some(path) = self.get_path(character.name().clone(), pos) {
    //             character.set_path(path.clone());
    //             Ok(path)
    //         } else {
    //             Err("Path not available".into())
    //         }
    //     } else {
    //         Err("Room/Region pair doesn't exist.".into())
    //     }
    // }

    pub fn objects(&self) -> Vec<&MapObject> {
        self.objects.iter().map(|f| f).collect()
    }
    pub fn regions(&self) -> Vec<&Region> {
        self.regions.iter().map(|r| r).collect()
    }
    // pub fn get_visible_objects(&self, character: &Character) -> HashMap<String, Vec<Coordinates>> {
    //     let (source_name, source_pos, v_range): (&String, &Coordinates, &i64) =
    //         (character.name(), character.position(), character.v_range());
    //     let mut visible_objects: HashMap<String, Vec<Coordinates>> = HashMap::new();

    //     for object in &self.objects {
    //         let object_pos = &object.position;
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
    //                 if let Some(collider) = &self.colliders[x as usize][y as usize] {
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

    //             if !obstructed {
    //                 visible_objects
    //                     .entry(object.name().clone())
    //                     .or_insert_with(Vec::new)
    //                     .push(object.position.clone());
    //             }
    //         }
    //     }

    //     visible_objects
    // }
    // pub fn set_object(
    //     &mut self,
    //     object_name: String,
    //     owner_name: Option<String>,
    // ) -> Result<(), Box<dyn Error>> {
    //     if let Some(o) = self
    //         .objects
    //         .iter_mut()
    //         .filter(|o| *o.name() == object_name)
    //         .nth(0)
    //     {
    //         o.set_owner(owner_name);
    //         return Ok(());
    //     }
    //     Err(("Object Doesn't Exist.".into()))
    // }
    pub async fn day_start(&mut self, llama: &Ollama, date: Date) {
        let navigator = &Navigator::new(&self);
        join_all(
            self.get_characters_mut()
                .iter_mut()
                .map(|f| f.day_start(llama, &date, navigator)),
        )
        .await;
    }
    pub fn ascend_all(&mut self) {
        let navigator = &Navigator::new(&self);
        self.get_characters_mut()
            .iter_mut()
            .for_each(|f| f.ascend(navigator));
    }
    pub async fn update(
        &mut self,
        datetime: &DateTime,
        llama: &Ollama,
    ) -> (bool, Vec<ActionEntry>) {
        // let (new_time, _) = self.datetime.1 + Time::from_seconds(TIME_STEP);
        // self.datetime.1 = new_time;
        let navigator = Navigator::new(self);
        // let objects_buffer = self.objects_buffer.clone();
        let results = join_all(
            self
                // .get_map_mut()
                .get_characters_mut()
                .iter_mut()
                .map(|f| f.tick(datetime, &navigator, llama)),
        )
        .await;
        let mut all_sleep: Vec<bool> = results.iter().map(|(sleep, _)| *sleep).collect();
        let entries: Vec<ActionEntry> = results.iter().filter_map(|(_, e)| e.clone()).collect();
        self.calculate_colliders();
        all_sleep.dedup();
        let all_sleep = if all_sleep.len() == 1 {
            *all_sleep.iter().nth(0).unwrap()
        } else {
            false
        };
        (all_sleep, entries)
    }
    // pub async fn test(&mut self, llama: &Ollama, datetime: &DateTime){
    //     self.characters.iter_mut().for_each(|f|{
    //         f.decide_room(llama, datetime, &self);
    //     });
    // }
    pub fn region_slice(&self) -> Vec<Region> {
        self.regions.clone()
    }
    pub fn room_slice(&self) -> Vec<Room> {
        let mut container = vec![];
        self.regions.iter().for_each(|f| {
            f.rooms()
                .iter()
                .filter(|r| r.region_name().is_some())
                .for_each(|r| container.push((*r).clone()))
        });
        container
    }
    pub fn collider_slice(&self) -> Vec<Vec<Option<String>>> {
        self.colliders.clone()
    }
    pub fn object_slice(&self) -> Vec<MapObject> {
        self.objects.clone()
    }
    pub fn size(&self) -> Coordinates {
        self.size.clone()
    }
    // pub fn map_info_slice(&self) -> (Vec<Region>, Vec<Room>, Vec<Vec<Option<String>>>){
    //     (self.region_slice(),)
    // }
    pub fn color(c: &char) -> String {
        match c.to_ascii_lowercase() {
            'a' => "⬛".into(),
            's' => "🧱".into(),
            'g' => "🏴‍☠️".into(),
            'o' => "🏢".into(),
            _ => "".into(),
        }
    }
}
impl Display for WorldMap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let regions = self
            .regions_as_chars()
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&c| format!("{} ", WorldMap::color(&c)))
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n");

        let colliders = self
            .as_chars()
            .iter()
            .map(|row| row.iter().map(|&c| format!("{} ", c)).collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");
        let rooms = self
            .room_slice()
            .iter()
            .map(|r| r.name())
            .collect::<Vec<String>>()
            .join("; ");
        write!(
            f,
            // "Regions:
            // {},
            "Colliders:\n{}",
            // Rooms: [{}]",
            // regions,
            colliders,
            // rooms
        )
    }
}
// pub struct MapInfoSlice(Vec<Region>, Vec<Room>, Vec<Vec<Option<String>>>);
