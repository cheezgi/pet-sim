
extern crate pet_sim;

use pet_sim::{Pet, Command};

fn main() {
    println!("Welcome to Pet Simulator!");

    // TODO: check for saved game

    print!("What kind of pet do you have? ");
    let kind = pet_sim::get_input();

    print!("What's its name? ");
    let name = pet_sim::get_input();

    let mut pet = Pet::new(kind, name);

    println!("Aww, {} the {}! How cute.", pet.name(), pet.kind());

    game_loop(pet);
}

fn game_loop(mut pet: Pet) {
    while !pet.is_dead() {
        print!("> ");

        if let Ok(cmd) = pet_sim::parse_command(pet_sim::get_input()) {
            match cmd {
                Command::Quit => {
                    // TODO: save game state
                    println!("Bye!");
                    break
                },
            }
        } else {
            println!("I didn't understand that.");
        }
    }
}

