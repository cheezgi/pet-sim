
extern crate pet_sim;

use pet_sim::{Pet, Command};

fn main() {
    println!("Welcome to Pet Simulator!");

    // TODO: check for saved game

    print!("What's your pet's name? ");
    let name = pet_sim::get_input();

    print!("What kind of pet is it? ");
    let kind = pet_sim::get_input();

    println!("Aww, {} the {}! How cute.", name, kind);

    let mut pet = Pet::new(kind, name);

    game_loop(&mut pet);

    if pet.is_dead() {
        println!("{} has passed away. R.I.P.\n\nFinal statistics:\n\n{}", pet.name(), pet);
    }
}

fn game_loop(pet: &mut Pet) {
    while !pet.is_dead() {
        print!("> ");

        if let Ok(cmd) = pet_sim::parse_command(&pet_sim::get_input()) {
            match cmd {
                Command::Empty => {}

                Command::Quit => {
                    // TODO: save game state
                    println!("Bye!");
                    break
                },

                Command::Stats => {
                    println!("{}", pet);
                },

                Command::Help => {
                    println!("Welcome to Pet Simulator. In this game, you take care of your pet, {0}.\n\
                              Use the following commands to help you in your quest to be the best owner:\n\
                              \tfeed <amount>: Feed {0}.\n\
                              \tplay <amount>: Play with {0}\n\
                              \twork <amount>: Gain money.\n\
                              \tgive <item>: Give an item to {0}\n\
                              \ttake <item>: Take an item from {0}\n\
                              I wish you and {0} luck. Have fun!", pet.name());
                },

                Command::Feed(amt) => {
                    println!("You feed {}. Yum!", pet.name());
                    pet.feed(amt);
                },

                Command::Play(amt) => {
                    println!("You play with {}. Whoo, haha!", pet.name());
                    pet.play(amt);
                },

                Command::Work(amt) => {
                    println!("You go to work. {} misses you.", pet.name());
                    for _ in 1..amt.scale(9) {
                        // TODO: give player money
                        pet.update();
                    }
                },

                Command::Bathe(amt) => {
                    println!("You give {} a bath. Scrub-a-dub dub!", pet.name());
                    pet.clean(amt);
                }

                _ => println!("unimplemented")
            }
        } else {
            println!("I didn't understand that.");
        }
    }
}

