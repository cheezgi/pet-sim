
use item::Item;
use clamp::Clamp;
use amt::Amount;

use std::fmt;

pub struct Pet {
    dead: bool,
    sick: bool,
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
            sick: false,
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

    pub fn update(&mut self) {
        if self.hunger <= 40 {
            self.damage(Amount::Bit);
        }

        if self.cleanliness <= 25 || self.hunger <= 35 {
            self.sicken();
        }

        if self.is_sick() {
            self.hunger(Amount::Some);
            self.damage(Amount::Bit);
            self.bore(Amount::Some);
        } else {
            self.hunger(Amount::Little);
            self.bore(Amount::Little);
        }
        self.dirty(Amount::Little);

        self.age();

        if self.age == 255 || self.health <= 10 {
            self.dead = true;
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

    pub fn is_sick(&self) -> bool {
        self.sick
    }

    pub fn sicken(&mut self) {
        self.sick = true;
    }

    pub fn medicate(&mut self) {
        self.sick = false;
    }

    pub fn give(&mut self, item: Item) {
        self.inventory.push(item);
    }

    pub fn take(&mut self, item: &Item) {
        if self.inventory.contains(item) {
            self.inventory.remove_item(item);
        }
    }

    pub fn feed(&mut self, amt: Amount) {
        let a = amt.scale(5);
        self.hunger = (self.hunger as i16 + a as i16).clamp(0, 100) as u8;
        self.heal(Amount::Little);
    }

    pub fn hunger(&mut self, amt: Amount) {
        let a = amt.scale(7);
        self.hunger = (self.hunger as i16 - a as i16).clamp(0, 100) as u8;
    }

    pub fn heal(&mut self, amt: Amount) {
        let a = amt.scale(5);
        self.health = (self.health as i16 + a as i16).clamp(0, 100) as u8;
    }

    pub fn damage(&mut self, amt: Amount) {
        let a = amt.scale(5);
        self.health = (self.health as i16 - a as i16).clamp(0, 100) as u8;
    }

    pub fn clean(&mut self, amt: Amount) {
        let a = amt.scale(5);
        self.cleanliness = (self.cleanliness as i16 + a as i16).clamp(0, 100) as u8;
        self.heal(Amount::Little);
    }

    pub fn dirty(&mut self, amt: Amount) {
        let a = amt.scale(5);
        self.cleanliness = (self.cleanliness as i16 - a as i16).clamp(0, 100) as u8;
    }

    pub fn play(&mut self, amt: Amount) {
        let a = amt.scale(5);
        self.happiness = (self.happiness as i16 + a as i16).clamp(0, 100) as u8;
        self.heal(Amount::Little);
    }

    pub fn bore(&mut self, amt: Amount) {
        let a = amt.scale(5);
        self.happiness = (self.happiness as i16 - a as i16).clamp(0, 100) as u8;
    }

    pub fn age(&mut self) {
        self.age = (self.age as i16 + 1i16).clamp(0, 255) as u8;
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

