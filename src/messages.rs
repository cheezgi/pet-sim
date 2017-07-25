
use pet::Pet;

pub struct Messages {
    sick: bool,
    unhealthy: bool,
    unhappy: bool,
    hungry: bool,
    dirty: bool,
}

impl Messages {
    pub fn new() -> Self {
        Messages {
            sick: false,
            unhealthy: false,
            unhappy: false,
            hungry: false,
            dirty: false,
        }
    }

    pub fn analyze(&mut self, pet: &Pet) {
        if pet.is_sick() && !self.sick {
            self.sick = true;
            println!("Your pet is sick! Be sure to take extra care of {}.", pet.name());
        }

        if pet.is_unhealthy() && !self.unhealthy {
            self.unhealthy = true;
            println!("Your pet is unhealthy! Be sure to clean, feed, and play with {}.", pet.name());
        }

        if pet.is_unhappy() && !self.unhappy {
            self.unhappy = true;
            println!("Your pet is unhappy! Give {} some attention.", pet.name());
        }

        if pet.is_hungry() && !self.hungry {
            self.hungry = true;
            println!("Your pet is very hungry! Give {} some food.", pet.name());
        }

        if pet.is_dirty() && !self.dirty {
            self.dirty = true;
            println!("Your pet is very dirty! Clean {} up.", pet.name());
        }
    }

    pub fn update(&mut self, pet: &Pet) {
        if !pet.is_sick() {
            if self.sick {
                println!("{} is no longer sick. Hooray!", pet.name());
            }
            self.sick = false;
        }

        if !pet.is_unhappy() {
            self.unhappy = false;
        }

        if !pet.is_hungry() {
            self.hungry = false;
        }

        if !pet.is_dirty() {
            self.dirty = false;
        }
    }
}

