
extern crate pet_sim;

use pet_sim::{Pet, Command};

fn main() {
    println!("Welcome to Pet Simulator!");

    // TODO: check for saved game

    print!("What kind of pet do you have? ");
    let kind = pet_sim::get_input();

    print!("What's its name? ");
    let name = pet_sim::get_input();

    println!("Aww, {} the {}! How cute.", name, kind);

    game_loop(Pet::new(kind, name));
}

fn game_loop(mut pet: Pet) {
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
                }

                _ => println!("unimplemented")
            }
        } else {
            println!("I didn't understand that.");
        }
    }
}

