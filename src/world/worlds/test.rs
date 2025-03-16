// use crate::personality::{action::Action, memory::short_term::ShortTerm};

// use crate::world::character::{Character, Direction, Placeholder};
// use crate::world::world::World;
// use crate::world::world_map::{Coordinates, MapObject, Region, Room, WorldMap};
// // pub fn test_char() -> Character {
// //     Character::new(
// //        22,
// //        vec!["Creative".to_string(), "Resourceful".to_string(), "Independent".to_string()],
// //        vec!["Laid-back".to_string(), "Ambitious".to_string(), "Practical".to_string()]
// //        ,
// //         "He spends his free time tinkering in his DIY workshop, experimenting with woodworking projects and home brewing, and dreaming of one day opening his own craft brewery or furniture making business.".to_string(),
// // "Living Room".to_string(),
// // ShortTerm::default(),
// // super::character::Placeholder::MALE,
// // "Man".to_string(),
// // Coordinates((2, 1)),
// // None,
// // super::character::Direction::NORTH,
// // vec![]
// //     )
// // }
// pub fn test_world() -> World {
//     World::new({
//         let mut map = WorldMap::new(Coordinates(30, 30));
//         let mut region = Region::new("1".to_string(), Coordinates(0, 0), Coordinates(10, 10));
//         let room = Room::new(
//             "Living Room".to_string(),
//             Coordinates(0, 0),
//             Coordinates(10, 10),
//             vec![Coordinates(9, 5), Coordinates(9, 6)],
//             None,
//         );
//         let (region_name, room_name) = (region.name(), room.name());
//         region.add_room(room);
//         map.add_region(region);
//         map.add_walls();
//         map.add_character(    Character::new(
//            22,
//            vec!["Creative".to_string(), "Resourceful".to_string(), "Independent".to_string()],
//            vec!["Laid-back".to_string(), "Ambitious".to_string(), "Practical".to_string()]
//            ,
//             "He spends his free time tinkering in his DIY workshop, experimenting with woodworking projects and home brewing, and dreaming of one day opening his own craft brewery or furniture making business.".to_string(),
//             (region_name, room_name),
//         ShortTerm::default(),
//         Placeholder::MALE,
//         "Man".to_string(),
//         Coordinates(2, 1),
//         Room::default(),
//         None,
//         Direction::NORTH,
//         vec![]
//         ));
//         map.add_object(MapObject::new(
//             4,
//             4,
//             // Rotation::N,
//             Coordinates(15, 15),
//             "Block".to_string(),
//             true,
//         ));
//         map.add_object(MapObject::new(
//             1,
//             7,
//             // Rotation::N,
//             Coordinates(1, 5),
//             "Glock".to_string(),
//             true,
//         ));
//         map.add_object(MapObject::new(
//             1,
//             1,
//             // Rotation::N,
//             Coordinates(2, 2),
//             "Vase".to_string(),
//             true,
//         ));
//         map.calculate_colliders();
//         map
//     })
// }
