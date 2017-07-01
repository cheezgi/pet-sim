
use item::Item;
use clamp::Clamp;

use std::fmt;

pub struct Pet {
    dead: bool,
    health: u8,
    happiness: u8,
    hunger: u8,
    age: u8,
    cleanliness: u8,
    kind: String,
    name: String,
    inventory: Vec<Item>,
}

impl Pet {
    pub fn new(kind: String, name: String) -> Pet {
        Pet {
            dead: false,
            health: 100,
            happiness: 70,
            hunger: 70,
            age: 1,
            cleanliness: 70,
            kind: kind,
            name: name,
            inventory: vec![],
        }
    }

    pub fn kind(&self) -> &str {
        &self.kind
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_dead(&self) -> bool {
        self.dead
    }

    pub fn give(&mut self, item: Item) {
        self.inventory.push(item);
    }

    pub fn take(&mut self, item: &Item) {
        if self.inventory.contains(item) {
            self.inventory.remove_item(item);
        }
    }

    pub fn feed(&mut self, amt: u8) {
        self.hunger += amt;
        self.hunger.clamp(0, 100);
    }

    pub fn behunger(&mut self, amt: u8) {
        self.hunger -= amt;
        self.hunger.clamp(0, 100);
    }

    pub fn heal(&mut self, amt: u8) {
        self.health += amt;
        self.health.clamp(0, 100);
    }

    pub fn damage(&mut self, amt: u8) {
        self.health -= amt;
        self.health.clamp(0, 100);
    }

    pub fn clean(&mut self, amt: u8) {
        self.cleanliness += amt;
        self.cleanliness.clamp(0, 100);
    }

    pub fn dirty(&mut self, amt: u8) {
        self.cleanliness -= amt;
        self.cleanliness.clamp(0, 100);
    }

    pub fn play(&mut self, amt: u8) {
        self.happiness += amt;
        self.happiness.clamp(0, 100);
    }

    pub fn bore(&mut self, amt: u8) {
        self.happiness -= amt;
        self.happiness.clamp(0, 100);
    }

    pub fn age(&mut self, amt: u8) {
        self.age += amt;
        self.age.clamp(0, 20);
    }
}

impl fmt::Display for Pet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} the {}:\n\thunger: {}\n\thealth: {}\n\
                   \tcleanliness: {}\n\thappiness: {}\n\tage: {}\n\tinventory: {:?}",
               self.name, self.kind, self.hunger, self.health, self.cleanliness,
               self.happiness, self.age, self.inventory)
    }
}

