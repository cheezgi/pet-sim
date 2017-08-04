
extern crate pet_sim;

extern crate serde;
extern crate serde_json;

use pet_sim::{Pet, Command, Messages, Player, Settings, read_settings};

fn main() {
    println!("Welcome to Pet Simulator!");

    // TODO: check for saved game
    let settings: Settings = read_settings();

    print!("What's your pet's name? ");
    let name = pet_sim::get_input();

    print!("What kind of pet is it? ");
    let kind = pet_sim::get_input();

    println!("Aww, {} the {}! How cute.", name, kind);

    let mut pet = Pet::new(kind, name, settings);
    let mut player = Player::new();

    game_loop(&mut pet, &mut player);

    if pet.is_dead() {
        println!("{} has passed away. R.I.P.\n\nFinal statistics:\n\n{}", pet.name(), pet);
    }
}

fn game_loop(pet: &mut Pet, player: &mut Player) {
    let mut messages = Messages::new();

    while !pet.is_dead() {
        print!("> ");

        if let Ok(cmd) = pet_sim::parse_command(&pet_sim::get_input()) {
            match cmd {
                Command::Empty => {},

                Command::Quit => {
                    // TODO: save game state
                    println!("Bye!");
                    break
                },

                Command::Stats => println!("{}", pet),

                Command::Help => println!("Welcome to Pet Simulator. In this game, you take care of your pet, {0}.\n\
                              Use the following commands to help you in your quest to be the best owner:\n\
                              \tfeed <amount>: Feed {0}.\n\
                              \tplay <amount>: Play with {0}\n\
                              \twork <amount>: Gain money.\n\
                              \tgive <item>: Give an item to {0}\n\
                              \ttake <item>: Take an item from {0}\n\
                              I wish you and {0} luck. Have fun!", pet.name()),

                Command::Xyzzy => println!("{} looks at you, confused. Nothing happens.", pet.name()),

                #[cfg(feature = "debug")]
                Command::Debug => {
                    println!("Debug");
                    println!("pet: {}\n", pet);
                    println!("player: {}\n", player);
                    println!("messages: {:?}", messages);
                }

                Command::Feed(amt) => {
                    println!("You feed {}. Yum!", pet.name());
                    pet.feed(amt);
                    pet.update_and_age();
                },

                Command::Play(amt) => {
                    println!("You play with {}. Whoo, haha!", pet.name());
                    pet.play(amt);
                    pet.update_and_age();
                },

                Command::Work(amt) => {
                    println!("You go to work. {} misses you.", pet.name());
                    for _ in 1..amt.scale(11) {
                        // TODO: give player money
                        pet.update();
                    }
                    pet.age();
                },

                Command::Bathe(amt) => {
                    println!("You give {} a bath. Scrub-a-dub dub!", pet.name());
                    pet.clean(amt);
                    pet.update_and_age();
                }

                _ => println!("unimplemented")
            }
        } else {
            println!("I didn't understand that.");
        }

        messages.analyze(pet);
        messages.update(pet);
    }
}

