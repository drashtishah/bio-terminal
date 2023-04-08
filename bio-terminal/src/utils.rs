use std::io::{self, Write};

use rosalind::{
    count_nucleotides, find_motif, hamming_distance, protein_mass, reverse_complement,
    transcribe_dna, translate_rna,
    utils::{FunctionResult, RosalindInputType},
};
use std::collections::HashMap;

pub enum RosalindInput {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
}

pub fn one_sequence_input(msg: &str) -> RosalindInputType {
    let mut input = String::new();
    print!("{}", msg);
    io::stdout().flush().expect("Error flushing stdout");
    io::stdin()
        .read_line(&mut input)
        .expect("There was an error while reading your input data.");
    input = input.trim().to_string();
    RosalindInputType::OneSequence(input)
}

pub fn two_sequence_input() -> RosalindInputType {
    let mut seq_array = ["Error".to_string(), "Error".to_string()];
    seq_array[0] = one_sequence_input("Enter the first sequence: ").unwrap_sequence();
    seq_array[1] = one_sequence_input("Enter the second sequence: ").unwrap_sequence();
    RosalindInputType::TwoSequence(seq_array)
}

pub fn two_sequence_input_v2() -> RosalindInputType {
    let mut seq_array = ["Error".to_string(), "Error".to_string()];
    seq_array[0] = one_sequence_input("Enter the DNA sequence: ").unwrap_sequence();
    seq_array[1] = one_sequence_input("Enter the motif you want to find: ").unwrap_sequence();
    RosalindInputType::TwoSequence(seq_array)
}

/// Prints the available Rosalind challenges.
pub fn print_challenges() {
    println!("{}", "1. Counting DNA Nucleotides");
    println!("{}", "2. Transcribing DNA into RNA");
    println!("{}", "3. Complementing a Strand of DNA");
    println!("{}", "4. Translating RNA into Protein");
    println!("{}", "5. Calculating Protein Mass");
    println!("{}", "6. Counting Point Mutations");
    println!("{}", "7. Finding a Motif in DNA");
}

/// Prints the available commands for the command line application.
pub fn print_commands() {
    println!("rosalind -> solve Rosalind challenges");
    println!("acoustics -> access data & tools for bioacoustics");
    println!("exit -> exit the terminal");
    println!("help -> show this command list again");
}

pub fn function_map() -> HashMap<i32, fn(RosalindInputType) -> FunctionResult> {
    let mut function_map: HashMap<i32, fn(RosalindInputType) -> FunctionResult> = HashMap::new();
    function_map.insert(1, count_nucleotides);
    function_map.insert(2, transcribe_dna);
    function_map.insert(3, reverse_complement);
    function_map.insert(4, translate_rna);
    function_map.insert(5, protein_mass);
    function_map.insert(6, hamming_distance);
    function_map.insert(7, find_motif);
    function_map
}
