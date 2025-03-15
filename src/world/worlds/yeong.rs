use crate::world::{
    world::World,
    world_map::{Coordinates, Region, Room, WorldMap},
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
        apartment.add_room(Room::new("Rm_001".to_string(), Coordinates(0, 7), Coordinates(7, 14), vec![Coordinates(6, 10), Coordinates(6, 11), Coordinates(6, 12)], Some(apartment.name().clone())));
        apartment.add_room(Room::new("Rm_002".to_string(), Coordinates(6, 0), Coordinates(10, 8), vec![Coordinates(7, 7), Coordinates(8, 7)], Some(apartment.name().clone())));
        map.add_region(apartment);
        map.add_region(street);
        map.add_region(gym);
        map.add_region(office);
        map.add_walls();
        map.calculate_colliders();
        map
    })
}
