#[derive(Debug)]
pub struct Player {
    pub name: String
}

impl Player {
    pub fn from_name(name: String) -> Player {
        Self {
            name
        }
    }
}
