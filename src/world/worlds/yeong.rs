use crate::world::{
    character::Character,
    world::World,
    world_map::{Coordinates, MapObject, Region, Room, WorldMap},
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
        apartment.add_room(Room::new(
            "Rm_001".to_string(),
            Coordinates(0, 7),
            Coordinates(7, 14),
            vec![Coordinates(6, 10), Coordinates(6, 11), Coordinates(6, 12)],
            Some(apartment.name().clone()),
        ));
        apartment.add_room(Room::new(
            "Rm_002".to_string(),
            Coordinates(6, 0),
            Coordinates(10, 8),
            vec![Coordinates(7, 7), Coordinates(8, 7)],
            Some(apartment.name().clone()),
        ));
        apartment.add_room(Room::new(
            "Common_Room".to_string(),
            Coordinates(6, 7),
            Coordinates(20, 14),
            vec![
                Coordinates(7, 20),
                Coordinates(8, 20),
                Coordinates(9, 20),
                Coordinates(11, 20),
                Coordinates(12, 20),
                Coordinates(16, 20),
                Coordinates(17, 20),
                Coordinates(18, 20),
                Coordinates(24, 20),
            ],
            Some(apartment.name().clone()),
        ));
        apartment.add_room(Room::new(
            "Exit".to_string(),
            Coordinates(6, 20),
            Coordinates(5, 12),
            vec![Coordinates(7, 31), Coordinates(8, 31), Coordinates(9, 31)],
            Some(apartment.name().clone()),
        ));
        apartment.add_room(Room::new(
            "Storage_Closet".to_string(),
            Coordinates(10, 20),
            Coordinates(4, 12),
            vec![],
            Some(apartment.name().clone()),
        ));
        apartment.add_room(Room::new(
            "Library".to_string(),
            Coordinates(13, 20),
            Coordinates(9, 12),
            vec![],
            Some(apartment.name().clone()),
        ));
        apartment.add_room(Room::new(
            "Toilet".to_string(),
            Coordinates(21, 20),
            Coordinates(5, 12),
            vec![],
            Some(apartment.name().clone()),
        ));

        gym.add_room(Room::new(
            "Gym".to_string(),
            Coordinates(10, 31),
            Coordinates(16, 10),
            vec![Coordinates(10, 38), Coordinates(10, 39)],
            Some(gym.name().clone()),
        ));

        office.add_room(Room::new(
            "Office".to_string(),
            Coordinates(10, 40),
            Coordinates(16, 11),
            vec![Coordinates(10, 48), Coordinates(10, 49)],
            Some(office.name().clone()),
        ));

        map.add_object(MapObject::new(
            2,
            1,
            Coordinates(13, 1),
            "Bed_001".to_string(),
            true,
        ));
        map.add_object(MapObject::new(
            1,
            2,
            Coordinates(7, 1),
            "Yoga_Mat_001".to_string(),
            false,
        ));
        map.add_object(MapObject::new(
            1,
            1,
            Coordinates(10, 1),
            "Plant_001".to_string(),
            true,
        ));
        map.add_object(MapObject::new(
            1,
            1,
            Coordinates(7, 3),
            "Candles_001".to_string(),
            true,
        ));
        map.add_object(MapObject::new(
            1,
            1,
            Coordinates(10, 6),
            "Photo_001".to_string(),
            false,
        ));
        map.add_object(MapObject::new(
            1,
            1,
            Coordinates(14, 3),
            "Desk_001".to_string(),
            true,
        ));
        map.add_object(MapObject::new(
            1,
            3,
            Coordinates(12, 6),
            "Bookshelf_001".to_string(),
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
            Coordinates(10, 3),
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
