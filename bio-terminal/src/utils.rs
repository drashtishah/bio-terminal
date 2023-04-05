use std::collections::HashMap;
use rosalind::{count_nucleotides, 
                transcribe_dna, 
                reverse_complement,
                translate_rna,
                protein_mass,
                utils::FunctionResult,
            };

pub fn print_challenges() {
    println!("{}", "1. Counting DNA Nucleotides");
    println!("{}", "2. Transcribing DNA into RNA");
    println!("{}", "3. Complementing a Strand of DNA");
    println!("{}", "4. Translating RNA into Protein");
    println!("{}", "5. Calculating Protein Mass");
}

pub fn print_commands() {
    println!("rosalind -> solve Rosalind challenges");
    println!("acoustics -> access data & tools for bioacoustics");
    println!("exit -> exit the terminal");
    println!("help -> show this command list again");
}

pub fn function_map() -> HashMap<i32, fn(&str) -> FunctionResult> {
    let mut function_map: HashMap<i32, fn(&str) -> FunctionResult> = HashMap::new();
    function_map.insert(1, count_nucleotides);
    function_map.insert(2, transcribe_dna);
    function_map.insert(3, reverse_complement);
    function_map.insert(4, translate_rna);
    function_map.insert(5, protein_mass);
    function_map
}
