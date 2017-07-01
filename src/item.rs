
#[derive(Debug, PartialEq, Clone)]
pub struct Item {
    pub edible: bool,
    pub name: String,
}

impl Item {
    pub fn new(name: String, edible: bool) -> Self {
        Item {
            edible: edible,
            name: name,
        }
    }
}

