use super::world_map::{Coordinates, Identity, Region, WorldMap, Character, Rotation, GenericObject};

pub fn test_world() -> WorldMap {
    let mut world = WorldMap::new(Coordinates((30, 30)));
    world.add_region(Region::new(
        "1".to_string(),
        Coordinates((0, 0)),
        Coordinates((10, 10)),
        vec![Coordinates((9, 5)), Coordinates((9, 6))],
    ));
    world.add_walls();
    world.add_character(Character::new(
        Identity::new(1, 1, Rotation::N, Coordinates((2, 1))),
        "Man".to_string(),
    ));
    world.add_object(GenericObject::new(
        Identity::new(4, 4, Rotation::N, Coordinates((15, 15))),
        "Block".to_string(),
        true,
    ));
    world.add_object(GenericObject::new(
        Identity::new(1, 7, Rotation::N, Coordinates((1, 5))),
        "Glock".to_string(),
        true,
    ));
    world.add_object(GenericObject::new(
        Identity::new(1, 1, Rotation::N, Coordinates((2, 2))),
        "Vase".to_string(),
        true,
    ));
    world.calculate_colliders();
    world
}
