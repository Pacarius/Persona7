use crate::personality::{action::Action, memory::short_term::ShortTerm};

use super::{
    character::Character,
    world_map::{Coordinates, MapObject, Region, Room, Rotation, WorldMap},
};
pub fn test_char() -> Character {
    Character::new(
       22,
       vec!["Creative".to_string(), "Resourceful".to_string(), "Independent".to_string()],
       vec!["Laid-back".to_string(), "Ambitious".to_string(), "Practical".to_string()]
       ,
        "He spends his free time tinkering in his DIY workshop, experimenting with woodworking projects and home brewing, and dreaming of one day opening his own craft brewery or furniture making business.".to_string(),
"Living Room".to_string(),
ShortTerm::default(),
super::character::Placeholder::MALE,
"Man".to_string(),
Coordinates((2, 1)),
None,
super::character::Direction::NORTH,
vec![]
    )
}
pub fn test_world() -> WorldMap {
    let mut world = WorldMap::new(Coordinates((30, 30)));
    let mut region = Region::new(
        "1".to_string(),
        Coordinates((0, 0)),
        Coordinates((10, 10)),
    );
    region.add_room(Room::new(
        "Living Room".to_string(),
        Coordinates((0, 0)),
        Coordinates((5, 5)),
        vec![Coordinates((9, 5)), Coordinates((9, 6))]
    ));
    world.add_region(region);
    world.add_walls();
    world.add_character(test_char());
    world.add_object(MapObject::new(
        4,
        4,
        Rotation::N,
        Coordinates((15, 15)),
        "Block".to_string(),
        true,
    ));
    world.add_object(MapObject::new(
        1,
        7,
        Rotation::N,
        Coordinates((1, 5)),
        "Glock".to_string(),
        true,
    ));
    world.add_object(MapObject::new(
        1,
        1,
        Rotation::N,
        Coordinates((2, 2)),
        "Vase".to_string(),
        true,
    ));
    world.calculate_colliders();
    world
}
