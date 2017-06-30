
extern crate pet_sim;

fn main() {
    println!("Welcome to Pet Simulator!");

    print!("What kind of pet do you have? ");
    let kind = pet_sim::get_input();

    print!("What's its name? ");
    let name = pet_sim::get_input();

    let mut pet = pet_sim::Pet::new(kind, name);

    pet.age(1);

    println!("{} the {}, aww!", pet.name(), pet.kind());
}

