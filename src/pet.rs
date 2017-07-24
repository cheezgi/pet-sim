
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

            // lower = bigger change, higher = smaller change
            feed_scale: 5,
            hunger_scale: 7,
            heal_scale: 5,
            damage_scale: 5,
            clean_scale: 2,
            dirty_scale: 5,
            play_scale: 4,
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

        if self.age == 255 || self.health <= 10 {
            self.dead = true;
        }
    }

    pub fn update_and_age(&mut self) {
        self.update();
        self.age();
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
        self.hunger = add_scale(self.hunger, amt, self.feed_scale);
        self.heal(Amount::Little);
    }

    pub fn hunger(&mut self, amt: Amount) {
        self.hunger = sub_scale(self.hunger, amt, self.hunger_scale);
    }

    pub fn heal(&mut self, amt: Amount) {
        self.health = add_scale(self.health, amt, self.heal_scale);
    }

    pub fn damage(&mut self, amt: Amount) {
        self.health = sub_scale(self.health, amt, self.damage_scale);
    }

    pub fn clean(&mut self, amt: Amount) {
        self.cleanliness = add_scale(self.cleanliness, amt, self.clean_scale);
        self.heal(Amount::Little);
    }

    pub fn dirty(&mut self, amt: Amount) {
        self.cleanliness = sub_scale(self.cleanliness, amt, self.dirty_scale);
    }

    pub fn play(&mut self, amt: Amount) {
        self.happiness = add_scale(self.happiness, amt, self.play_scale);
        self.heal(Amount::Little);
    }

    pub fn bore(&mut self, amt: Amount) {
        self.happiness = sub_scale(self.happiness, amt, self.bore_scale);
    }

    pub fn age(&mut self) {
        self.age = (self.age as i16 + 1i16).clamp(0, 255) as u8;
    }
}

fn add_scale(lhs: u8, rhs: Amount, scale: u8) -> u8 {
    (lhs as i16 + rhs.scale(scale) as i16).clamp(0, 100) as u8
}

fn sub_scale(lhs: u8, rhs: Amount, scale: u8) -> u8 {
    (lhs as i16 - rhs.scale(scale) as i16).clamp(0, 100) as u8
}

impl fmt::Display for Pet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} the {}:\n\thunger: {}\n\thealth: {}\n\
                   \tcleanliness: {}\n\thappiness: {}\n\tage: {}\n\tinventory: {:?}",
               self.name, self.kind, self.hunger, self.health, self.cleanliness,
               self.happiness, self.age, self.inventory)
    }
}

