use std::fmt::Display;

use crate::Coordinates;

#[derive(Debug, Clone)]
pub struct MapObject {
    vertical: i64,
    horizontal: i64,
    // rotation: Rotation,
    position: Coordinates,
    name: String,
    collision: bool,
    action: Option<String>,
    region: String,
    room: String,
    owner: Option<String>,
}
impl MapObject {
    pub fn new(
        vertical: i64,
        horizontal: i64,
        // rotation: Rotation,
        position: Coordinates,
        name: String,
        collision: bool,
    ) -> Self {
        Self {
            vertical,
            horizontal,
            // rotation,
            position,
            name,
            collision,
            action: None,
            region: "".to_string(),
            room: "".to_string(),
            owner: None,
        }
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn action(&self) -> &Option<String> {
        &self.action
    }
    pub fn late(&mut self, region: String, room: String) {
        // if let Some((region, room)) = map.get_position_info(&self.position){
        self.region = region;
        self.room = room;
        // }
    }
    pub fn collision(&self) -> bool {
        self.collision
    }
    pub fn room(&self) -> String {
        self.room.clone()
    }
    pub fn region(&self) -> String {
        self.region.clone()
    }
    pub fn position(&self) -> &Coordinates {
        &self.position
    }
    pub fn owner(&self) -> &Option<String> {
        &self.owner
    }
    pub fn set_owner(&mut self, name: Option<String>) {
        self.owner = name;
    }
}
#[derive(Debug, Clone)]
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
    pub fn rooms(&self) -> Vec<&Room> {
        self.rooms.iter().map(|r| r).collect()
    }
    pub fn position(&self) -> &Coordinates {
        &self.position
    }
    pub fn size(&self) -> &Coordinates {
        &self.size
    }
}
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Room {
    //Position: (XTop, YTop); Size: (XSize, YSize)
    name: String,
    position: Coordinates,
    size: Coordinates,
    holes: Vec<Coordinates>,
    region_name: Option<String>,
    walled: bool,
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
            walled: true,
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
        walled: bool,
    ) -> Self {
        Self {
            name,
            position,
            size,
            holes,
            region_name,
            walled,
        }
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn position(&self) -> &Coordinates {
        &self.position
    }
    pub fn size(&self) -> &Coordinates {
        &self.size
    }
    pub fn holes(&self) -> &Vec<Coordinates> {
        &self.holes
    }
    pub fn region_name(&self) -> &Option<String> {
        &self.region_name
    }
    pub fn is_walled(&self) -> bool {
        self.walled
    }
}