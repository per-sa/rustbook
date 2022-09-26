// elevator system using rust
extern crate colored;
use colored::*;

fn main() {
    let mut current_floor = 0;
    let mut destination_floor = 0;

    println!("Welcome to the elevator system");
    println!("You are at {} floor", current_floor.to_string().green());
    println!("Where do you want to go?");

    loop {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        destination_floor = input.trim().parse().expect("Please type a number!");

        if destination_floor > current_floor {
            println!("Going up");
            for i in current_floor..destination_floor {
                println!("Floor {}", i.to_string().green());
            }
        } else if destination_floor < current_floor {
            println!("Going down");
            for i in (destination_floor..current_floor).rev() {
                println!("Floor {}", i.to_string().red());
            }
        } else {
            println!(
                "You are already at {} floor",
                current_floor.to_string().green()
            );
        }

        current_floor = destination_floor;
        println!("You are at {} floor", current_floor.to_string().green());
        println!("Where do you want to go?");
    }
}
