use std::io;

fn main() {
    //bmi_calculator();
    tip_calculator();
}

fn bmi_calculator() {
    println!("BMI Calculator");
    let mut weight_input = String::new();
    println!("Enter your weight in lbs: ");
    io::stdin().read_line(&mut weight_input).expect("Failed to read line");
    let mut height_input = String::new();
    println!("Enter your height in inches: ");
    io::stdin().read_line(&mut height_input).expect("Failed to read line");

    let weight: f32 = weight_input.trim().parse().expect("Please type a number!");
    let height: f32 = height_input.trim().parse().expect("Please type a number!");

    let bmi = (weight / (height * height)) * 703.0;
    println!("Your BMI is: {}", bmi);
    if bmi < 18.5 {
        println!("You are underweight");
    } else if bmi >= 18.5 && bmi < 24.9 {
        println!("You are normal weight");
    } else if bmi >= 25.0 && bmi < 29.9 {
        println!("You are overweight");
    } else {
        println!("You are obese");
    }
}


fn tip_calculator() {
    println!("Tip Calculator");
    let mut bill_input = String::new();
    println!("Enter the bill amount: ");
    io::stdin().read_line(&mut bill_input).expect("Failed to read line");
    let mut tip_input = String::new();
    println!("Enter the tip percentage: ");
    io::stdin().read_line(&mut tip_input).expect("Failed to read line");

    let bill: f32 = bill_input.trim().parse().expect("Please type a number!");
    let tip: f32 = tip_input.trim().parse().expect("Please type a number!");

    let tip_amount = bill * (tip / 100.0);
    let total = bill + tip_amount;
    println!("The tip amount is: {}", tip_amount);
    println!("The total amount is: {}", total);
}
