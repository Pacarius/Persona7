use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum Placeholder {
    MALE,
    FEMALE,
}

#[derive(Debug, Deserialize)]
pub enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

#[derive(Debug, Deserialize)]
pub struct Character {
    pub placeholder: Placeholder,
    pub name: String,
    pub location: (usize, usize),
    pub path: Option<Vec<(usize, usize)>>,
    pub direction: Direction,
}

impl Character {
    pub fn new(
        placeholder: Placeholder,
        name: String,
        location: (usize, usize),
        direction: Direction,
    ) -> Self {
        Character {
            placeholder,
            name,
            location,
            path: None,
            direction,
        }
    }

    pub fn override_path(&mut self, path: Vec<(usize, usize)>) {
        self.path = Some(path);
    }

    pub fn get_path(&self) -> Option<&Vec<(usize, usize)>> {
        self.path.as_ref()
    }
}
