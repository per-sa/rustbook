use colored::*;
use std::io;

fn main() {
    println!("Hello,  welcome to the BMI Calculator!");
    println!(
        "Please enter your weight in {} (ex: 78):",
        "kilograms".cyan()
    );

    let mut weight = String::new();

    io::stdin()
        .read_line(&mut weight)
        .expect("Failed to read line");

    let weight: f32 = match weight.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0,
    };

    println!(
        "Please enter your height in {} (ex: 1.80):",
        "meters".cyan()
    );

    let mut height = String::new();

    io::stdin()
        .read_line(&mut height)
        .expect("Failed to read line");

    let height: f32 = match height.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0,
    };

    let bmi = weight / (height * height);

    println!("Your BMI is: {}", bmi.to_string().green());
}
