pub mod utils;
use std::io::{self, Write};
use utils::{print_challenges, function_map};

fn main() {
    let mut x = 0;
    let solutions = function_map();
    let welcome_message: &str = "This is your bio-terminal.\nYou can analyse biological data here.\nWould you like to start by solving challenges on Rosalind?\nPlease enter Y to start.\nOr enter exit to close the bio-terminal.";

    loop {
        if x == 0 {
            println!("{}", welcome_message);
        }

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("There was an error while reading your input data.");
        input = input.trim().to_string();

        if input == "exit" {
            break;
        } else if input == "Y" || input == "Yes" || input == "y" {
            x = x + 1;
            print_challenges();
            print!("Enter a number: ");
            io::stdout().flush().expect("Error flushing stdout");
        } else if input == "N" || input == "No" || input == "n" {
            println!("{}", "Bio-Terminal is a work-in-progress. You can only solve Rosalind challenges right now.");
            break;
        }

        let num: i32 = input.parse().unwrap_or_default();
        if num > 0 {
            if let Some(func) = solutions.get(&num) {
                print!("Enter the input sequence: ");
                io::stdout().flush().expect("Error flushing stdout");
                input = "".to_string();
                io::stdin()
                .read_line(&mut input)
                .expect("There was an error while reading your input data.");
                input = input.trim().to_string();
                let result = func(&input);
                println!("{:?}", result);
        }

        x = x + 1;
    }
}
}