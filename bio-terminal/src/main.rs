pub mod utils;
use std::io::{self, Write};
use utils::{print_challenges, function_map};

fn main() {
    let mut x = 0;
    let solutions = function_map();
    let mut input_list = Vec::new();
    let welcome_message: &str = "This is your bio-terminal.\nYou can analyse biological data here.\nWould you like to start by solving challenges on Rosalind?\nPlease enter Y to start.\nOr enter exit to close the bio-terminal.";

    loop {
        if x == 0 {
            // Print the welcome message only once
            println!("{}", welcome_message);
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
        } else if input == "Y" {
            println!("{}", "Select the problem you want to solve.");
            print_challenges();
            print!("Enter a number: ");
            io::stdout().flush().expect("Error flushing stdout");
            let temp_input = input.clone();
            input_list.push(temp_input);
        } else if input == "N" {
            println!("{}", "Bio-Terminal is a work-in-progress. You can only solve Rosalind challenges right now.");
            break;
        } else if num == 0 {
            // Any other text input
            println!("Please check your input.");
        }

        let yes: String = "Y".to_string();
        if num > 0 && input_list.contains(&yes) {
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
                println!("Your solution for question {} -> {:?}", num, result);
                println!("Please select the next challenge you want to solve.");
                print_challenges();
            }  else {
                println!("This question does not seem to be in the list.");
            }
        } else if num > 0 {
            // For numeric inputs typed in before a "Y"
            println!("Please check your input.");
        }
}
}