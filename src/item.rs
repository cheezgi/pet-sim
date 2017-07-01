
#[derive(Debug, PartialEq, Clone)]
pub struct Item {
    pub size: u8,
    pub name: String,
}

impl Item {
    pub fn new(name: String, size: u8) -> Self {
        Item {
            size: size,
            name: name,
        }
    }
}

