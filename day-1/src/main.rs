use std::io;
fn main() {
    print_bread_making_steps();
    change_variables();
    user_input();
}

fn print_bread_making_steps() {
    println!("1. Mix 500g of Flour, 10g Yeast and 300ml Water in a bowl.");
    println!("2. Knead the dough for 10 minutes.");
    println!("3. Let the dough rest for 30 minutes.");
    println!("4. Bake the dough at 200°C for 30 minutes.");
    println!("5. Enjoy your bread!");
}

fn change_variables() {
    let mut glass1 = "milk";
    let mut glass2 = "juice";
    glass1 = glass2;
    println!("glass1: {}", glass1);
}

fn user_input() {
    println!("Band name generator!");
    println!("Please enter the name of the city you grew up in:");
    let mut name_of_city = String::new();
    io::stdin().read_line(&mut name_of_city).expect("Failed to read line");
    println!("Please enter the name of your first pet:");
    let mut name_of_pet = String::new();
    io::stdin().read_line(&mut name_of_pet).expect("Failed to read line");
    println!("Your band name is: {} {}", name_of_city.trim(), name_of_pet.trim());
}

