use std::{
    any::Any, collections::HashMap, fmt::{Debug, Display}, hash::Hash
};

#[derive(PartialEq)]
pub struct Coordinates((usize, usize));
pub struct Character<'a>(Identity, String);
pub struct GenericObject(Identity, String);
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
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.name())
//     }
// }
pub struct Identity {
    //Common stats for map items / characters
    len: i64,
    width: i64,
    rotation: Rotation,
    location: Coordinates,
}
#[derive(PartialEq)]
enum Rotation {
    N,
    E,
    S,
    W,
}
pub struct Region {
    //Position: (XTop, YTop); Size: (XSize, YSize)
    name: String,
    position: Coordinates,
    size: Coordinates,
    holes: Vec<Coordinates>,
}
pub struct WorldMap<'a> {
    size: Coordinates,
    regions: Vec<Region>,
    objects: Vec<GenericObject>,
    characters: Vec<Character<'a>>,
    walls: Vec<Coordinates>,
    pub colliders: Vec<Vec<Option<String>>>,
    pub character_list: HashMap<String, &'a Character<'a>>
}
impl WorldMap<'_> {
    pub fn new(size: Coordinates) -> Self {
        let (x, y) = (size.0 .0, size.0 .1);
        Self {
            size,
            regions: Vec::new(),
            objects: Vec::new(),
            characters: Vec::new(),
            walls: Vec::new(),
            character_list: HashMap::new(),
            colliders: vec![vec![None; x]; y],
        }
    }
    pub fn add_region(&mut self, region: Region) {
        self.regions.push(region);
    }
    pub fn add_walls(&mut self) {
        for region in &self.regions {
            let (x_top, y_top) = (region.position.0 .0, region.position.0 .1);
            let (x_size, y_size) = (region.size.0 .0, region.size.0 .1);

            // Add top and bottom walls
            for x in x_top..(x_top + x_size) {
                let top_wall = Coordinates((x, y_top));
                let bottom_wall = Coordinates((x, y_top + y_size - 1));
                if !region.holes.contains(&top_wall) {
                    self.walls.push(top_wall);
                }
                if !region.holes.contains(&bottom_wall) {
                    self.walls.push(bottom_wall);
                }
            }

            // Add left and right walls
            for y in y_top..(y_top + y_size) {
                let left_wall = Coordinates((x_top, y));
                let right_wall = Coordinates((x_top + x_size - 1, y));
                if !region.holes.contains(&left_wall) {
                    self.walls.push(left_wall);
                }
                if !region.holes.contains(&right_wall) {
                    self.walls.push(right_wall);
                }
            }
        }
    }
    pub fn calculate_colliders(&mut self) {
        for w in &self.walls {
            self.colliders[w.0 .0][w.0 .1] = Some("Wall".to_string());
        }
        for o in &self.objects {
            let identity = &o.0;
            let Coordinates((x, y)) = identity.location;
            let len = identity.len as usize;
            let width = identity.width as usize;

            match identity.rotation {
                Rotation::N | Rotation::S => {
                    for i in 0..len {
                        for j in 0..width {
                            let x_pos = if identity.rotation == Rotation::N {
                                x - i
                            } else {
                                x + i
                            };
                            let y_pos = y + j;
                            if self.colliders[x_pos][y_pos].is_none() {
                                self.colliders[x_pos][y_pos] = Some(o.1.clone());
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
                Rotation::E | Rotation::W => {
                    for i in 0..width {
                        for j in 0..len {
                            let x_pos = x + i;
                            let y_pos = if identity.rotation == Rotation::E {
                                y + j
                            } else {
                                y - j
                            };
                            if self.colliders[x_pos][y_pos].is_none() {
                                self.colliders[x_pos][y_pos] = Some(o.1.clone());
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
            }
        }
        for c in &self.characters {
            let (identity, name) = (&c.0, &c.1);
            let Coordinates((x, y)) = identity.location;
            if self.colliders[x][y].is_none() {
                self.colliders[x][y] = Some(c.1.clone());
                self.character_list.insert(c.1.clone(), c);
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
    pub fn add_object(&mut self, object: GenericObject) {
        self.objects.push(object);
    }
    pub fn add_character(&mut self, character: Character) {
        self.characters.push(character);
    }
    pub fn test_world() -> Self{
        let mut world = WorldMap::new(Coordinates((30, 30)));
        world.add_region(Region {
            name: "1".to_string(),
            position: Coordinates((0, 0)),
            size: Coordinates((10, 10)),
            holes: vec![Coordinates((9, 5)), Coordinates((9, 6))],
        });
        world.add_walls();
        world.add_character(Character {
            0: Identity {
                len: 1,
                width: 1,
                rotation: Rotation::E,
                location: Coordinates((1, 1)),
            },
            1: "Man".to_string(),
        });
        world.add_object(GenericObject {
            0: Identity {
                len: 4,
                width: 4,
                rotation: Rotation::E,
                location: Coordinates { 0: (15, 15) },
            },
            1: "Block".to_string(),
        });
        world.calculate_colliders();
        world
    }
}
impl Display for WorldMap<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        {
            let c = self
                .colliders
                .iter()
                .map(|m| {
                    m.iter()
                        .map(|o| {
                            if o.is_some() {
                                o.clone().unwrap()
                            } else {
                                "___".to_string()
                            }
                        })
                        .collect::<Vec<String>>()
                        .join("___")
                })
                .collect::<Vec<String>>()
                .join("\n");
            write!(f, "{}", c)
        }
    }
}
