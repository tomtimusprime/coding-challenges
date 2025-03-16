use std::io;
use std::process;
use rand::Rng;

fn main() {
    //treasure_island();
    //lists();
    rock_paper_scissors();
}

fn treasure_island() {
    println!("Welcome to Treasure Island!");
    println!("Your mission is to find the treasure.");
    println!("You're at a crossroad. Where do you want to go? Type 'left' or 'right'.");
    let mut direction = String::new();
    io::stdin().read_line(&mut direction).expect("Failed to read line");
    let direction_lower = direction.trim().to_lowercase();
    if direction_lower == "left" {
        println!("You've come to a lake. There is an island in the middle of the lake.");
        println!("Type 'wait' to wait for a boat. Type 'swim' to swim across.");
        let mut lake_action = String::new();
        io::stdin().read_line(&mut lake_action).expect("Failed to read line");
        let lake_action_lower = lake_action.trim().to_lowercase();
        if lake_action_lower == "wait" {
            println!("You arrive at the island unharmed. There is a house with 3 doors.");
            println!("One red, one yellow, and one blue. Which color do you choose?");
            let mut door_color = String::new();
            io::stdin().read_line(&mut door_color).expect("Failed to read line");
            let door_color_lower = door_color.trim().to_lowercase();
            if door_color_lower == "red" {
                println!("It's a room full of fire. Game Over.");
                process::exit(0);
            }
            if door_color_lower == "yellow" {
                println!("You found the treasure! You win!");
                process::exit(0);
            }
            if door_color_lower == "blue" {
                println!("You enter a room of beasts. Game Over.");
                process::exit(0);
            }
        }
        if lake_action_lower == "swim" {
            println!("You were attacked by a trout. Game Over.");
            process::exit(0);
        }
    }
    if direction_lower == "right" {
        println!("You fell into a hole. Game Over.");
        process::exit(0);
    }
}

fn lists() {
    let states_of_america = vec!["Washington", "Utah", "California", "Texas", "New York"];
    for state in states_of_america {
        println!("{}", state);
    }
}

fn rock_paper_scissors() {
    println!("Welcome to Rock, Paper, Scissors!");
    println!("What do you choose? Type 'rock', 'paper', or 'scissors'.");
    let mut user_choice = String::new();
    io::stdin().read_line(&mut user_choice).expect("Failed to read line");
    let user_choice_lower = user_choice.trim().to_lowercase();
    let choices = vec!["rock", "paper", "scissors"];
    let mut random = rand::thread_rng();
    let random_index = random.gen_range(0..3);
    let computer_choice = choices[random_index];
    println!("You chose: {}", user_choice_lower);
    println!("Computer chose: {}", computer_choice);
    if user_choice_lower == "rock" && computer_choice == "scissors" {
        println!("You win!");
    } else if user_choice_lower == "rock" && computer_choice == "paper" {
        println!("You lose!");
    } else if user_choice_lower == "paper" && computer_choice == "rock" {
        println!("You win!");
    } else if user_choice_lower == "paper" && computer_choice == "scissors" {
        println!("You lose!");
    } else if user_choice_lower == "scissors" && computer_choice == "paper" {
        println!("You win!");
    } else if user_choice_lower == "scissors" && computer_choice == "rock" {
        println!("You lose!");
    } else {
        println!("It's a draw!");
    }
}
