
use item::Item;

pub struct Player {
    money: i32,
    inventory: Vec<Item>,
    story: (), // TODO: add story progress marker
}

impl Player {
    pub fn new() -> Self {
        Player {
            money: 500,
            inventory: vec![],
            story: (),
        }
    }

    pub fn money(&self) -> i32 {
        self.money
    }

    pub fn deposit(&mut self, amt: i32) {
        self.money += amt;
    }

    pub fn withdraw(&mut self, amt: i32) {
        self.money -= amt;
    }
}

