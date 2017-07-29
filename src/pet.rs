
use item::Item;
use clamp::Clamp;
use amt::Amount;
use save::Settings;
use ::{sub_scale, add_scale};

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

    hunger_damage: u8,
    hunger_sicken: u8,
    cleanliness_sicken: u8,
    health_kill: u8,
    age_kill: u8,

    health_message: u8,
    hunger_message: u8,
    bore_message: u8,
    clean_message: u8,
}

impl Pet {
    pub fn new(kind: String, name: String, settings: Settings) -> Pet {
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
            feed_scale: settings.feed_scale,
            hunger_scale: settings.hunger_scale,
            heal_scale: settings.heal_scale,
            damage_scale: settings.damage_scale,
            clean_scale: settings.clean_scale,
            dirty_scale: settings.dirty_scale,
            play_scale: settings.play_scale,
            bore_scale: settings.bore_scale,

            hunger_damage: settings.hunger_damage,
            hunger_sicken: settings.hunger_sicken,
            cleanliness_sicken: settings.cleanliness_sicken,
            age_kill: settings.age_kill,
            health_kill: settings.health_kill,

            health_message: settings.health_message,
            hunger_message: settings.hunger_message,
            bore_message: settings.bore_message,
            clean_message: settings.clean_message,
        }
    }

    pub fn update(&mut self) {
        if self.hunger <= self.hunger_damage {
            self.damage(Amount::Bit);
        }

        if self.hunger <= self.hunger_sicken || self.cleanliness <= self.cleanliness_sicken {
            self.sicken();
        } else {
            self.medicate();
        }

        if self.is_sick() {
            self.hunger(Amount::Some);
            self.bore(Amount::Some);
            self.damage(Amount::Little);
        } else {
            self.hunger(Amount::Little);
            self.bore(Amount::Little);
            self.heal(Amount::Bit);
        }
        self.dirty(Amount::Little);

        if self.age == self.age_kill || self.health <= self.health_kill {
            self.dead = true;
        }
    }

    pub fn age(&mut self) {
        self.age = (self.age as i16 + 1i16).clamp(0, 255) as u8;
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

    pub fn is_unhealthy(&self) -> bool {
        self.health <= self.health_message
    }

    pub fn is_unhappy(&self) -> bool {
        self.happiness <= self.bore_message
    }

    pub fn is_hungry(&self) -> bool {
        self.hunger <= self.hunger_message
    }

    pub fn is_dirty(&self) -> bool {
        self.cleanliness <= self.clean_message
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
}

impl fmt::Display for Pet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} the {}{}:\n\thunger: {}\n\thealth: {}\n\
                   \tcleanliness: {}\n\thappiness: {}\n\tage: {}\n\tinventory: {:?}",
               self.name, self.kind,
               if self.sick {" (SICK)"} else {""},
               self.hunger, self.health, self.cleanliness,
               self.happiness, self.age, self.inventory)
    }
}

