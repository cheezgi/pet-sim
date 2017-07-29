
use item::Item;
use amt::Amount;
use ::{sub_scale, add_scale};

pub struct Player {
    money: i32,
    inventory: Vec<Item>,
    story: (), // TODO: add story progress marker

    deposit_scale: u8,
    withdraw_scale: u8,
}

impl Player {
    pub fn new() -> Self {
        Player {
            money: 500,
            inventory: vec![],
            story: (),

            deposit_scale: 5,
            withdraw_scale: 5,
        }
    }

    pub fn money(&self) -> i32 {
        self.money
    }

    pub fn deposit(&mut self, amt: Amount) -> i32 {
        let r = amt.scale(self.deposit_scale) as i32;
        self.money = self.money + r;
        r
    }

    pub fn withdraw(&mut self, amt: Amount) -> i32 {
        let r = amt.scale(self.withdraw_scale) as i32;
        self.money = self.money - r;
        r
    }
}

