pub mod utils;
use std::io::{self, Write};
use utils::{print_challenges, function_map, print_commands};

fn main() {
    let mut x = 0;
    let solutions = function_map();
    let mut input_list = Vec::new();
    let welcome_message: &str = "\nThis is your bio-terminal. You can analyse biological data here.";

    loop {
        if x == 0 {
            // Print the welcome message only once
            println!("{}", welcome_message);
            println!("This a list of commands you can use to navigate the terminal.\n");
            print_commands();
            println!("");
            x = x + 1;
        }

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("There was an error while reading your input data.");
        input = input.trim().to_string();
        let num: i32 = input.parse().unwrap_or_default();

        if input == "exit" {
            break;
        } else if input == "rosalind" {
            println!("\nPlease select the challenge you want to solve.");
            print_challenges();
            print!("\nEnter a number or a command: ");
            io::stdout().flush().expect("Error flushing stdout");
            let temp_input = input.clone();
            input_list.push(temp_input);
        } else if input == "acoustics" {
            println!("Select a function from this list to analyze bioacoustics data.");
        } else if input == "help" {
            println!("\nHere is the list of commands you can use to navigate the terminal.");
            print_commands();
            println!("");
        } else if num == 0 {
            // Any other text input
            println!("Please check your input.");
            println!("");
        }

        let rosalind: String = "rosalind".to_string();
        if num > 0 && input_list.contains(&rosalind) {
            if let Some(func) = solutions.get(&num) {
                print!("Enter the input sequence: ");
                io::stdout().flush().expect("Error flushing stdout");
                input = "".to_string();
                io::stdin()
                .read_line(&mut input)
                .expect("There was an error while reading your input data.");
                input = input.trim().to_string();
                if input == "exit" {break;}
                let result = func(&input);
                println!("Your solution for question {} -> {:?}\n", num, result);
                println!("Please select the next challenge you want to solve.");
                print_challenges();
                print!("\nEnter a number or a command: ");
                io::stdout().flush().expect("Error flushing stdout");
            }  else {
                println!("This challenge does not seem to be in the list.");
                println!("\nPlease select one of the challenges given below.");
                print_challenges();
                print!("\nEnter a number or a command: ");
                io::stdout().flush().expect("Error flushing stdout");
            }
        } else if num > 0 {
            // For numeric inputs typed in before a "Y"
            println!("Please check your input.");
            println!("");
        }
}
}