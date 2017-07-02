
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

    feed_scale: u8,
    hunger_scale: u8,
    heal_scale: u8,
    damage_scale: u8,
    clean_scale: u8,
    dirty_scale: u8,
    play_scale: u8,
    bore_scale: u8,
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

            feed_scale: 5,
            hunger_scale: 7,
            heal_scale: 5,
            damage_scale: 5,
            clean_scale: 5,
            dirty_scale: 5,
            play_scale: 5,
            bore_scale: 5,
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
        self.hunger = (self.hunger as i16 + amt.scale(self.feed_scale) as i16).clamp(0, 100) as u8;
        self.heal(Amount::Little);
    }

    pub fn hunger(&mut self, amt: Amount) {
        self.hunger = (self.hunger as i16 - amt.scale(self.hunger_scale) as i16).clamp(0, 100) as u8;
    }

    pub fn heal(&mut self, amt: Amount) {
        self.health = (self.health as i16 + amt.scale(self.heal_scale) as i16).clamp(0, 100) as u8;
    }

    pub fn damage(&mut self, amt: Amount) {
        self.health = (self.health as i16 - amt.scale(self.damage_scale) as i16).clamp(0, 100) as u8;
    }

    pub fn clean(&mut self, amt: Amount) {
        self.cleanliness = (self.cleanliness as i16 + amt.scale(self.clean_scale) as i16).clamp(0, 100) as u8;
        self.heal(Amount::Little);
    }

    pub fn dirty(&mut self, amt: Amount) {
        self.cleanliness = (self.cleanliness as i16 - amt.scale(self.dirty_scale) as i16).clamp(0, 100) as u8;
    }

    pub fn play(&mut self, amt: Amount) {
        self.happiness = (self.happiness as i16 + amt.scale(self.play_scale) as i16).clamp(0, 100) as u8;
        self.heal(Amount::Little);
    }

    pub fn bore(&mut self, amt: Amount) {
        self.happiness = (self.happiness as i16 - amt.scale(self.bore_scale) as i16).clamp(0, 100) as u8;
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

