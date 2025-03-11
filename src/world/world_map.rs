use core::time;
use std::collections::{HashSet, VecDeque};
use std::marker::PhantomData;

use std::{
    any::Any,
    collections::HashMap,
    fmt::{Debug, Display},
    hash::Hash,
};

use crate::misc::time::Time;

use super::character::Character;

#[derive(PartialEq, Debug)]
pub struct Coordinates(pub (usize, usize));
// pub struct Character(Identity, String);
#[derive(Debug)]
pub struct MapObject{
    len: i64,
    width: i64,
    rotation: Rotation,
    location: Coordinates,
    name: String,
    collision: bool,
    action: Option<String>
}
impl MapObject{
    pub fn new(len: i64, width: i64, rotation: Rotation, location: Coordinates, name: String, collision: bool) -> Self {
        Self {
            len,
            width,
            rotation,
            location,
            name,
            collision,
            action: None
        }
    }
    pub fn name(&self) -> &String{&self.name}
    pub fn action(&self) -> &Option<String>{&self.action}
}
// (Identity, String, bool);
// impl MapObject for GenericObject{
//     fn identity(&self) -> &Identity {
//         &self.0
//     }
//     fn name(&self) -> String {
//         self.1.clone()
//     }
// }
// impl MapObject for Character{
//     fn identity(&self) -> &Identity {
//         &self.0
//     }
//     fn name(&self) -> String {
//         self.1.clone()
//     }
// }
// pub trait MapObject{
//     fn identity(&self) -> &Identity;
//     fn name(&self) -> String;
// }
// impl Debug for dyn MapObject{
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "{}", self.name())
//     }
// }
// pub struct Identity {
//     //Common stats for map items / characters
//     len: i64,
//     width: i64,
//     rotation: Rotation,
//     location: Coordinates,
// }
// impl Identity {
//     pub fn new(len: i64, width: i64, rotation: Rotation, location: Coordinates) -> Self {
//         Self {
//             len,
//             width,
//             rotation,
//             location,
//         }
//     }
// }
#[derive(PartialEq, Debug)]
//For some reason colliders don't work with rotations when the world is being set up because it's AI generated code let's be real I'm not writing that
pub enum Rotation {
    N,
    E,
    S,
    W,
}
pub struct Region{
    name: String,
    position: Coordinates,
    size: Coordinates,
    rooms: Vec<Room>
}
impl Display for Region{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
impl Default for Region{
    fn default() -> Self {
        Self { name: "Void".to_string(), position: Coordinates((0, 0)), size: Coordinates((0, 0)), rooms: vec![] }
    }
}
impl Region{
    pub fn new(name: String, position: Coordinates, size: Coordinates) -> Self {
        Self {
            name,
            position,
            size,
            rooms: Vec::new(),
        }
    }
    pub fn add_room(&mut self, room: Room){self.rooms.push(room);}
}
pub struct Room {
    //Position: (XTop, YTop); Size: (XSize, YSize)
    name: String,
    position: Coordinates,
    size: Coordinates,
    holes: Vec<Coordinates>,
}
impl Display for Room{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
impl Default for Room{
    fn default() -> Self {
        Self { name: "Void".to_string(), position: Coordinates((0, 0)), size: Coordinates((0, 0)), holes: vec![] }
    }
}
impl Room {
    pub fn new(
        name: String,
        position: Coordinates,
        size: Coordinates,
        holes: Vec<Coordinates>,
    ) -> Self {
        Self {
            name,
            position,
            size,
            holes,
        }
    }
}
// impl Character {
//     pub fn new(identity: Identity, name: String) -> Self {
//         Self(identity, name)
//     }
// }
// impl MapObject {
//     pub fn new(identity: Identity, name: String, is_solid: bool) -> Self {
//         Self(identity, name, is_solid)
//     }
// }
pub struct WorldMap {
    size: Coordinates,
    regions: Vec<Region>,
    objects: Vec<MapObject>,
    characters: Vec<Character>,
    walls: Vec<Coordinates>,
    pub colliders: Vec<Vec<Option<String>>>,
}
impl WorldMap {
    pub fn new(size: Coordinates) -> Self {
        let (x, y) = (size.0 .0, size.0 .1);
        Self {
            size,
            regions: Vec::new(),
            objects: Vec::new(),
            characters: Vec::new(),
            walls: Vec::new(),
            colliders: vec![vec![None; x]; y],
        }
    }
    pub fn add_region(&mut self, region: Region) {
        self.regions.push(region);
    }
    pub fn add_walls(&mut self) {
        for region in &self.regions {
            for room in &region.rooms{
            let (x_top, y_top) = (room.position.0 .0, room.position.0 .1);
            let (x_size, y_size) = (room.size.0 .0, room.size.0 .1);

            // Add top and bottom walls
            for x in x_top..(x_top + x_size) {
                let top_wall = Coordinates((x, y_top));
                let bottom_wall = Coordinates((x, y_top + y_size - 1));
                if !room.holes.contains(&top_wall) {
                    self.walls.push(top_wall);
                }
                if !room.holes.contains(&bottom_wall) {
                    self.walls.push(bottom_wall);
                }
            }

            // Add left and right walls
            for y in y_top..(y_top + y_size) {
                let left_wall = Coordinates((x_top, y));
                let right_wall = Coordinates((x_top + x_size - 1, y));
                if !room.holes.contains(&left_wall) {
                    self.walls.push(left_wall);
                }
                if !room.holes.contains(&right_wall) {
                    self.walls.push(right_wall);
                }
            }
        }
        }
    }
    pub fn calculate_colliders(&mut self) {
        for w in &self.walls {
            self.colliders[w.0 .0][w.0 .1] = Some("Wall".to_string());
        }
        for o in &self.objects {
            if !o.collision {
                continue;
            }
            let Coordinates((y, x)) = o.location;
            let len = o.len as usize;
            let width = o.width as usize;

            for i in 0..len {
                for j in 0..width {
                    let (x_pos, y_pos) = match o.rotation {
                        //The rotation code doesn't work LOL
                        //I'm just spamming copilot for this because I just need a functional 2d world.
                        //I promise I'll make the frontend slightly less scuffed (maybe)
                        Rotation::N => (x - i, y + j),
                        Rotation::S => (x + i, y + j),
                        Rotation::E => (x + i, y + j),
                        Rotation::W => (x - i, y - j),
                    };
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
            let Coordinates((y, x)) = c.location;
            if self.colliders[x][y].is_none() {
                self.colliders[x][y] = Some(c.name.clone());
            } else {
                println!(
                    "Collider already exists at ({}, {}): {}",
                    x,
                    y,
                    self.colliders[x][y].as_ref().unwrap()
                );
            }
        }
    }
    pub fn add_object(&mut self, object: MapObject) {
        self.objects.push(object);
    }
    pub fn add_character(&mut self, character: Character) {
        if self.characters.iter().any(|f| f.name == character.name) {
            println!("{} already exists.", character.name);
        }
        self.characters.push(character);
    }
    fn as_chars(&self) -> Vec<Vec<char>> {
        self.colliders
            .iter()
            .map(|row| {
                row.iter()
                    .map(|cell| {
                        if let Some(ref obj) = cell {
                            obj.chars().next().unwrap()
                        } else {
                            ' '
                        }
                    })
                    .collect::<Vec<char>>()
            })
            .collect::<Vec<Vec<char>>>()
    }
}
impl Display for WorldMap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        {
            let c = self
                .colliders
                .iter()
                .map(|m| {
                    m.iter()
                        .map(|o| {
                            if let Some(ref obj) = o {
                                format!("{} ", obj.chars().next().unwrap())
                            } else {
                                "  ".to_string()
                            }
                        })
                        .collect::<String>()
                })
                .collect::<Vec<String>>()
                .join("\n");
            write!(f, "{}", c)
        }
    }
}
impl WorldMap {
    fn get_character(&self, name: String) -> &Character {
        self.characters
            .iter()
            .filter(|n| n.name.eq(&name))
            .nth(0)
            .unwrap()
    }
    pub fn get_characters(&self) -> Vec<&Character> {
        self.characters.iter().collect()
    }
    pub fn get_path(&self, name: String, target: Coordinates) -> Option<Vec<Coordinates>> {
        let start = &self.get_character(name).location;
        let goal = &target;

        let (start_x, start_y) = (start.0 .1, start.0 .0);
        let (goal_x, goal_y) = (goal.0 .1, goal.0 .0);

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut came_from = HashMap::new();

        queue.push_back((start_x, start_y));
        visited.insert((start_x, start_y));

        while let Some((x, y)) = queue.pop_front() {
            if (x, y) == (goal_x, goal_y) {
                let mut path = Vec::new();
                let mut current = (goal_x, goal_y);

                while current != (start_x, start_y) {
                    path.push(Coordinates(current));
                    current = came_from[&current];
                }

                path.push(Coordinates((start_x, start_y)));
                path.reverse();
                return Some(path);
            }

            for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let new_x = (x as isize + dx) as usize;
                let new_y = (y as isize + dy) as usize;

                if new_x < self.size.0 .0
                    && new_y < self.size.0 .1
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
    //Test function.
    pub fn get_path_visual(&self, name: String, target: Coordinates) -> String {
        let path = self.get_path(name, target);
        if path.is_none() {
            "No Path.".to_string()
        } else {
            let source = self.as_chars();
            let mut visual_map = source.clone();
            for Coordinates((x, y)) in path.unwrap() {
                visual_map[x][y] = '*';
            }
            visual_map
                .iter()
                .map(|row| row.iter().collect::<String>())
                .collect::<Vec<String>>()
                .join("\n")
        }
    }
    pub fn get_visible_colliders(&self, name: String, range: usize) -> Vec<String> {
        let character = self.get_character(name);
        let Coordinates((start_x, start_y)) = character.location;
        let mut visible_colliders = HashSet::new();
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back((start_x, start_y));
        visited.insert((start_x, start_y));

        while let Some((x, y)) = queue.pop_front() {
            if ((x as isize - start_x as isize).abs() as usize)
                + ((y as isize - start_y as isize).abs() as usize)
                > range
            {
                continue;
            }

            if let Some(ref collider) = self.colliders[x][y] {
                if collider != "Wall" && collider != &character.name {
                    visible_colliders.insert(collider.clone());
                }
            }

            for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let new_x = (x as isize + dx) as usize;
                let new_y = (y as isize + dy) as usize;

                if new_x < self.size.0 .0
                    && new_y < self.size.0 .1
                    && !visited.contains(&(new_x, new_y))
                {
                    if self.colliders[new_x][new_y].is_none()
                        || self.colliders[new_x][new_y].as_ref().unwrap() != "Wall"
                    {
                        queue.push_back((new_x, new_y));
                    }
                    visited.insert((new_x, new_y));
                }
            }
        }

        visible_colliders.into_iter().collect()
    }
}
