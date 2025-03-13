use std::io;

fn main() {
    pizza_delivery();
}


fn pizza_delivery() {
    println!("Welcome to Rust Pizza Deliveries!");
    println!("Please input the size of the pizza you would like to order: ");
    let mut price = 0;
    let mut size = "";
    let mut size_input = String::new();
    io::stdin().read_line(&mut size_input)
        .expect("Failed to read line");
    let size_lower = size_input.trim().to_lowercase();
    if size_lower == "s" {
        size = "small";
        price = 15;
    } else if size_lower == "m" {
        size = "medium";
        price = 20;
    } else if size_lower == "l" {
        size = "large";
        price = 25;
    } else {
        println!("Invalid input. Please enter 's', 'm', or 'l'.");
    }
    println!("Please enter whether you want pepperoni on your pizza: y/n");
    let mut pepperoni = false;
    let mut pepperoni_input = String::new();
    io::stdin().read_line(&mut pepperoni_input)
        .expect("Failed to read line");
    let pepperoni_lower = pepperoni_input.trim().to_lowercase();
    if pepperoni_lower == "y" {
        pepperoni = true;
        price += 5;
    } else if pepperoni_lower == "n" {
        pepperoni = false;
    } else {
        println!("Invalid input. Please enter 'y' or 'n'.");
    }

    println!("Do you want extra cheese? y/n");
    let mut cheese = false;
    let mut cheese_input = String::new();
    io::stdin().read_line(&mut cheese_input)
        .expect("Failed to read line");
    let cheese_lower = cheese_input.trim().to_lowercase();
    if cheese_lower == "y" {
        cheese = true;
        price += 5;
    } else if cheese_lower == "n" {
        cheese = false;
    } else {
        println!("Invalid input. Please enter 'y' or 'n'.");
    }

    println!("Your order is a {} pizza with pepperoni and extra cheese.", size);
    println!("The total price is ${}.", price);
    println!("Thank you for ordering from Rust Pizza Deliveries!");
}