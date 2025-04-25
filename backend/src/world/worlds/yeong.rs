use crate::world::{
    character::Character,
    utils::{MapObject, Region, Room},
    world::World,
    world_map::{Coordinates, WorldMap},
};

pub fn yeong() -> World {
    World::new({
        let mut map = WorldMap::new(Coordinates(200, 200));
        let mut apartment = Region::new(
            "Apartment".to_string(),
            Coordinates(0, 0),
            Coordinates(32, 32),
        );
        let mut street = Region::new("Street".to_string(), Coordinates(6, 32), Coordinates(4, 19));
        let mut gym = Region::new("Gym".to_string(), Coordinates(10, 32), Coordinates(16, 8));
        let mut office = Region::new(
            "Office".to_string(),
            Coordinates(10, 40),
            Coordinates(16, 11),
        );
        street.add_room(Room::new(
            "Street".into(),
            Coordinates(6, 32),
            Coordinates(4, 19),
            vec![],
            Some(street.name().clone()),
            false,
        ));
        apartment.add_room(Room::new(
            "Rm_001".to_string(),
            Coordinates(0, 7),
            Coordinates(7, 14),
            vec![Coordinates(6, 10), Coordinates(6, 11), Coordinates(6, 12)],
            Some(apartment.name().clone()),
            true,
        ));
        apartment.add_room(Room::new(
            "Rm_002".to_string(),
            Coordinates(6, 0),
            Coordinates(10, 8),
            vec![Coordinates(7, 7), Coordinates(8, 7)],
            Some(apartment.name().clone()),
            true,
        ));
        apartment.add_room(Room::new(
            "Living Room".to_string(),
            Coordinates(6, 7),
            Coordinates(20, 14),
            vec![
                Coordinates(7, 20),
                Coordinates(8, 20),
                // Coordinates(9, 20),
                Coordinates(11, 20),
                Coordinates(12, 20),
                Coordinates(16, 20),
                Coordinates(17, 20),
                Coordinates(18, 20),
                Coordinates(24, 20),
            ],
            Some(apartment.name().clone()),
            true,
        ));
        apartment.add_room(Room::new(
            "Exit".to_string(),
            Coordinates(6, 20),
            Coordinates(5, 12),
            vec![Coordinates(7, 31), Coordinates(8, 31), Coordinates(9, 31)],
            Some(apartment.name().clone()),
            true,
        ));
        apartment.add_room(Room::new(
            "Storage_Closet".to_string(),
            Coordinates(10, 20),
            Coordinates(4, 12),
            vec![],
            Some(apartment.name().clone()),
            true,
        ));
        apartment.add_room(Room::new(
            "Library".to_string(),
            Coordinates(13, 20),
            Coordinates(9, 12),
            vec![],
            Some(apartment.name().clone()),
            true,
        ));
        apartment.add_room(Room::new(
            "Toilet".to_string(),
            Coordinates(21, 20),
            Coordinates(5, 12),
            vec![],
            Some(apartment.name().clone()),
            true,
        ));

        gym.add_room(Room::new(
            //Please go boxing ring.
            "Boxing Ring".to_string(),
            Coordinates(10, 31),
            Coordinates(16, 10),
            vec![Coordinates(10, 38), Coordinates(10, 39)],
            Some(gym.name().clone()),
            true,
        ));

        office.add_room(Room::new(
            "Reception Area".to_string(),
            Coordinates(10, 40),
            Coordinates(6, 11),
            vec![Coordinates(10, 48), Coordinates(10, 49)],
            Some(office.name().clone()),
            true,
        ));
        office.add_room(Room::new(
            "Main Area".to_string(),
            Coordinates(15, 40),
            Coordinates(6, 11),
            vec![Coordinates(15, 41), Coordinates(15, 42)],
            Some(office.name().clone()),
            true,
        ));
        office.add_room(Room::new(
            "Office Area".to_string(),
            Coordinates(20, 40),
            Coordinates(6, 6),
            vec![Coordinates(20, 41), Coordinates(20, 42)],
            Some(office.name().clone()),
            true,
        ));
        office.add_room(Room::new(
            "Director's Office".to_string(),
            Coordinates(20, 45),
            Coordinates(6, 6),
            vec![Coordinates(23, 45), Coordinates(24, 45)],
            Some(office.name().clone()),
            true,
        ));

        //Room 1
        map.add_object(MapObject::new(
            1,
            2,
            Coordinates(13, 1),
            "Bed".to_string(),
            false,
        ));
        map.add_object(MapObject::new(
            1,
            2,
            Coordinates(7, 1),
            "Yoga_Mat".to_string(),
            false,
        ));
        map.add_object(MapObject::new(
            1,
            1,
            Coordinates(10, 1),
            "Plant".to_string(),
            true,
        ));
        map.add_object(MapObject::new(
            1,
            1,
            Coordinates(7, 3),
            "Candles".to_string(),
            true,
        ));
        map.add_object(MapObject::new(
            1,
            1,
            Coordinates(10, 6),
            "Photo".to_string(),
            false,
        ));
        map.add_object(MapObject::new(
            1,
            1,
            Coordinates(14, 3),
            "Desk".to_string(),
            true,
        ));
        map.add_object(MapObject::new(
            1,
            3,
            Coordinates(12, 6),
            "Bookshelf".to_string(),
            true,
        ));
        // Living Room
        map.add_object(MapObject::new(
            1,
            3,
            Coordinates(13, 8),
            "Couch".into(),
            false,
        ));
        map.add_object(MapObject::new(
            1,
            3,
            Coordinates(17, 8),
            "Bean Bags".into(),
            false,
        ));
        map.add_object(MapObject::new(1, 5, Coordinates(14, 11), "TV".into(), true));
        // Library
        map.add_object(MapObject::new(
            1,
            2,
            Coordinates(14, 21),
            "Philosophy Bookshelf".into(),
            true,
        ));
        map.add_object(MapObject::new(
            1,
            2,
            Coordinates(19, 21),
            "Science Bookshelf".into(),
            true,
        ));
        map.add_object(MapObject::new(
            5,
            1,
            Coordinates(14, 24),
            "Interstellar Bookshelf".into(),
            true,
        ));
        map.add_object(MapObject::new(
            1,
            4,
            Coordinates(14, 30),
            "Second Interstellar Bookshelf".into(),
            true,
        ));
        map.add_object(MapObject::new(
            1,
            1,
            Coordinates(20, 30),
            "Lamp".into(),
            true,
        ));
        map.add_object(MapObject::new(
            1,
            1,
            Coordinates(20, 24),
            "Chair 1".into(),
            false,
        ));
        map.add_object(MapObject::new(
            1,
            1,
            Coordinates(20, 28),
            "Chair 2".into(),
            false,
        ));
        map.add_object(MapObject::new(
            1,
            1,
            Coordinates(20, 26),
            "Coffee Table".into(),
            true,
        ));
        // Storage Room
        map.add_object(MapObject::new(
            1,
            1,
            Coordinates(12, 30),
            "Broom".into(),
            true,
        ));
        map.add_object(MapObject::new(
            2,
            1,
            Coordinates(11, 26),
            "Closet".into(),
            true,
        ));
        map.add_object(MapObject::new(
            1,
            1,
            Coordinates(11, 23),
            "Werid Stack of Boxes".into(),
            true,
        ));
        // Toilet(?)
        map.add_object(MapObject::new(
            1,
            1,
            Coordinates(22, 21),
            "Sink".into(),
            true,
        ));
        map.add_object(MapObject::new(
            1,
            1,
            Coordinates(22, 26),
            "Toilet".into(),
            false,
        ));
        map.add_object(MapObject::new(
            3,
            2,
            Coordinates(22, 28),
            "Shower".into(),
            false,
        ));
        // Office_Reception
        map.add_object(MapObject::new(
            4,
            1,
            Coordinates(13, 44),
            "Reception Desk".into(),
            true,
        ));
        // Office_Main
        map.add_object(MapObject::new(
            2,
            1,
            Coordinates(16, 44),
            "Printer_1".into(),
            true,
        ));
        map.add_object(MapObject::new(
            2,
            1,
            Coordinates(19, 44),
            "Printer_2".into(),
            true,
        ));
        map.add_object(MapObject::new(
            1,
            2,
            Coordinates(16, 48),
            "Office Desk".into(),
            true,
        ));
        // map.add_object(MapObject::new(
        //     1,
        //     2,
        //     Coordinates(16, 48),
        //     "Office Desk".into(),
        //     true,
        // ));
        // Office_Sub
        map.add_object(MapObject::new(
            2,
            1,
            Coordinates(23, 41),
            "Office Desk 1".into(),
            true,
        ));
        map.add_object(MapObject::new(
            1,
            1,
            Coordinates(22, 44),
            "Office Desk 2".into(),
            true,
        ));

        // Office_Manager
        //God I should really write a generator for this
        map.add_object(MapObject::new(
            1,
            3,
            Coordinates(21, 48),
            "DESK".into(),
            true,
        ));

        map.add_character(Character::new(
            28,
        vec!["Analytical".to_string(), "Methodical".to_string(), "Detail-oriented".to_string()], 
        vec!["Disciplined".to_string(), "Focused".to_string(), "Responsible".to_string()], 
        "She works as a data analyst, spending most of her free time reading scientific papers and attending conferences to learn more about her field.".to_string(), // lifestyle
            "Rm_001".to_string(), 
            crate::world::character::Placeholder::FEMALE,
        "Ava Thompson".to_string(), // name
            Coordinates(12, 3),
            5,
            0
            ));
        map.add_region(apartment);
        map.add_region(street);
        map.add_region(gym);
        map.add_region(office);
        map.add_walls();
        map.calculate_colliders();
        map
    })
}
