use core::time;
use std::collections::{HashSet, VecDeque};
use std::future;
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

use super::character::Character;

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
//AT SOME POINT THE X,Y COORDINATES OF EVERYTHING FLIPPED KILL ME
pub struct Coordinates(pub usize, pub usize);
impl Display for Coordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}
// pub struct Character(Identity, String);
#[derive(Debug)]
pub struct MapObject {
    horizontal: i64,
    vertical: i64,
    // rotation: Rotation,
    location: Coordinates,
    name: String,
    collision: bool,
    action: Option<String>,
}
impl MapObject {
    pub fn new(
        horizontal: i64,
        vertical: i64,
        // rotation: Rotation,
        location: Coordinates,
        name: String,
        collision: bool,
    ) -> Self {
        Self {
            horizontal,
            vertical,
            // rotation,
            location,
            name,
            collision,
            action: None,
        }
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn action(&self) -> &Option<String> {
        &self.action
    }
}
#[derive(Debug)]
pub struct Region {
    name: String,
    position: Coordinates,
    size: Coordinates,
    rooms: Vec<Room>,
}
impl Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
impl Default for Region {
    fn default() -> Self {
        Self {
            name: "Void".to_string(),
            position: Coordinates(0, 0),
            size: Coordinates(0, 0),
            rooms: vec![],
        }
    }
}
impl Region {
    pub fn new(name: String, position: Coordinates, size: Coordinates) -> Self {
        Self {
            name,
            position,
            size,
            rooms: Vec::new(),
        }
    }
    pub fn add_room(&mut self, mut room: Room) {
        if room.region_name.is_none() {
            room.region_name = Some(self.name.clone())
        }
        self.rooms.push(room);
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
}
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Room {
    //Position: (XTop, YTop); Size: (XSize, YSize)
    name: String,
    position: Coordinates,
    size: Coordinates,
    holes: Vec<Coordinates>,
    pub region_name: Option<String>,
}
impl Display for Room {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
impl Default for Room {
    fn default() -> Self {
        Self {
            name: "Void".to_string(),
            position: Coordinates(0, 0),
            size: Coordinates(0, 0),
            holes: vec![],
            region_name: Some("Void".to_string()),
        }
    }
}
impl Room {
    pub fn new(
        name: String,
        position: Coordinates,
        size: Coordinates,
        holes: Vec<Coordinates>,
        region_name: Option<String>,
    ) -> Self {
        Self {
            name,
            position,
            size,
            holes,
            region_name,
        }
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
}
pub struct WorldMap {
    size: Coordinates,
    regions: Vec<Region>,
    objects: Vec<MapObject>,
    characters: Vec<Character>,
    walls: Vec<Coordinates>,
    pub colliders: Vec<Vec<Option<String>>>,
    //This is fucking stupid but I'm out of ideas on how to do this elegantly.
    // pub room_region_map: HashMap<Room, String>
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
            // room_region_map: HashMap::new()
        }
    }
    pub fn add_region(&mut self, region: Region) {
        self.regions.push(region);
    }
    pub fn add_walls(&mut self) {
        let mut holes = vec![];
        for region in &self.regions {
            for room in &region.rooms {
                let (x_top, y_top) = (room.position.0, room.position.1);
                let (x_size, y_size) = (room.size.0, room.size.1);
                holes.extend(&room.holes);
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
            // holes.dedup();
            self.walls.retain(|w| !holes.contains(&w));
        }
    }
    //EXTREMELY INEFFICIENT BUT I'M OUT OF TIME, I'M JUST GONNA CALL THIS ONCE EVERY SERVER UPDATE FRAME WOOOOOO
    //NOOOOOOO I'M GONNA ONLY DO THIS ONCE AND ADD ANOTHER THING WHICH ONLY DOES CHARACTERS
    pub fn calculate_colliders(&mut self) {
        self.colliders = vec![vec![None; self.size.0]; self.size.1];
        for w in &self.walls {
            self.colliders[w.0][w.1] = Some("Wall".to_string());
        }
        for o in &self.objects {
            if !o.collision {
                continue;
            }
            let Coordinates(x, y) = o.location;
            let vertical = o.vertical as usize;
            let horizontal = o.horizontal as usize;

            for i in 0..vertical {
                for j in 0..horizontal {
                    let (x_pos, y_pos) = (x - i, y + j);
                    if self.colliders[x_pos][y_pos].is_none() {
                        self.colliders[x_pos][y_pos] = Some(o.name.clone());
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
        }
        for c in &self.characters {
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
        }
    }
    pub fn move_characters(&mut self) -> Vec<(Coordinates, Coordinates)> {
        let mut moved_positions = Vec::new();

        self.characters.iter_mut().for_each(|c| {
            let pos = c.position().clone();
            if let Some(pos) = c._move(){
                if self.colliders[pos.0.0][pos.0.1] == Some(c.name().clone()) {
                    self.colliders[pos.0.0][pos.0.1] = None;
                    self.colliders[pos.1.0][pos.1.1] = Some(c.name().clone()); 
                }
                moved_positions.push(pos);
            }
        });
        moved_positions
    }
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
            let Coordinates(x_top, y_top) = region.position;
            let Coordinates(x_size, y_size) = region.size;

            for x in x_top..(x_top + x_size) {
                for y in y_top..(y_top + y_size) {
                    if x < self.size.0 && y < self.size.1 {
                        region_map[y][x] = region.name.chars().next().unwrap_or(' ');
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
    pub fn get_characters(&self) -> Vec<&Character> {
        self.characters.iter().collect()
    }
    pub fn get_characters_mut(&mut self) -> Vec<&mut Character> {
        self.characters.iter_mut().collect()
    }
    // pub fn get_character(&self, name: String) -> &Character{
    //     self.characters.iter().filter(|f| f.name == name).nth(0).unwrap()
    // }
    pub fn get_path(&self, name: String, target: Coordinates) -> Option<VecDeque<Coordinates>> {
        let start = &self.get_character(name).position();
        let goal = &target;

        let (start_x, start_y) = (start.0, start.1);
        let (goal_x, goal_y) = (goal.0, goal.1);

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
                    && self.colliders[new_x][new_y].is_none()
                    && !visited.contains(&(new_x, new_y))
                {
                    queue.push_back((new_x, new_y));
                    visited.insert((new_x, new_y));
                    came_from.insert((new_x, new_y), (x, y));
                }
            }
        }

        None
    }
    pub fn get_path_visual(&self, name: String) -> String {
        let character = self.get_character(name.clone());
        let path = match character.path() {
            Some(p) => p,
            None => return "No path available.".to_string(),
        };

        let mut map = self.as_chars();

        for Coordinates(x, y) in path {
            if x < &self.size.0 && y < &self.size.1 {
                map[*y][*x] = '*';
            }
        }

        map.iter()
            .map(|row| row.iter().map(|&c| format!("{} ", c)).collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    }
    pub fn set_path(&mut self, name: String, position: Coordinates) {
        match self.get_path(name.clone(), position) {
            Some(o) => self.get_character_mut(name).set_path(o),
            None => println!("No paths available."),
        }
        // self.get_character_mut(name).set_path(self.get_path(name, position));
    }
    pub fn get_position_info(&self, position: Coordinates) -> Option<(String, String)> {
        for region in &self.regions {
            if position.0 >= region.position.0
                && position.0 < region.position.0 + region.size.0
                && position.1 >= region.position.1
                && position.1 < region.position.1 + region.size.1
            {
                for room in &region.rooms {
                    if position.0 >= room.position.0
                        && position.0 < room.position.0 + room.size.0
                        && position.1 >= room.position.1
                        && position.1 < room.position.1 + room.size.1
                    {
                        return Some((region.name(), room.name()));
                    }
                }
            }
        }
        None
    }

    pub fn get_visible_objects(&self, character: &Character) -> HashMap<String, Vec<Coordinates>> {
        let (source_name, source_pos, v_range): (&String, &Coordinates, &i64) =
            (character.name(), character.position(), character.v_range());
        let mut visible_objects: HashMap<String, Vec<Coordinates>> = HashMap::new();

        for object in &self.objects {
            let object_pos = &object.location;
            let distance = ((object_pos.0 as i64 - source_pos.0 as i64).pow(2)
                + (object_pos.1 as i64 - source_pos.1 as i64).pow(2))
            .isqrt() as i64;
        // println!("{}", distance);
            if distance <= *v_range {
                let mut obstructed = false;
                let (x0, y0) = (source_pos.0 as i64, source_pos.1 as i64);
                let (x1, y1) = (object_pos.0 as i64, object_pos.1 as i64);

                // Bresenham's line algorithm to check for walls
                let dx = (x1 - x0).abs();
                let dy = -(y1 - y0).abs();
                let mut err = dx + dy;
                let mut x = x0;
                let mut y = y0;

                let sx = if x0 < x1 { 1 } else { -1 };
                let sy = if y0 < y1 { 1 } else { -1 };

                while x != x1 || y != y1 {
                    if let Some(collider) = &self.colliders[x as usize][y as usize] {
                        if collider == "Wall" {
                            obstructed = true;
                            break;
                        }
                    }

                    let e2 = 2 * err;
                    if e2 >= dy {
                        err += dy;
                        x += sx;
                    }
                    if e2 <= dx {
                        err += dx;
                        y += sy;
                    }
                }

                if !obstructed {
                    visible_objects
                        .entry(object.name().clone())
                        .or_insert_with(Vec::new)
                        .push(object.location.clone());
                }
            }
        }

        visible_objects
    }
    pub async fn day_start(&mut self, llama: &Ollama, date: Date) {
        join_all(
            self.get_characters_mut()
                .iter_mut()
                .map(|f| f.day_start(llama, &date)),
        )
        .await;
    }
    pub async fn update(&mut self, time: Time) {
        // let (new_time, _) = self.datetime.1 + Time::from_seconds(TIME_STEP);
        // self.datetime.1 = new_time;
        join_all(
            self
                // .get_map_mut()
                .get_characters_mut()
                .iter_mut()
                .map(|f| f.tick(&time)),
        )
        .await;
        self.calculate_colliders();
    }
}
impl Display for WorldMap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let regions = self
            .regions_as_chars()
            .iter()
            .map(|row| row.iter().map(|&c| format!("{} ", c)).collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");

        let colliders = self
            .as_chars()
            .iter()
            .map(|row| row.iter().map(|&c| format!("{} ", c)).collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "Regions:\n{}\n\nColliders:\n{}", regions, colliders)
    }
}
