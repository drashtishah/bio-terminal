use std::collections::HashMap;
use rosalind::{count_nucleotides, transcribe_dna, FunctionResult};

pub fn print_challenges() {
    println!("{}", "1. Counting DNA Nucleotides");
    println!("{}", "2. Transcribing DNA into RNA");
}

pub fn function_map() -> HashMap<i32, fn(&str) -> FunctionResult> {
    let mut function_map: HashMap<i32, fn(&str) -> FunctionResult> = HashMap::new();
    function_map.insert(1, count_nucleotides);
    function_map.insert(2, transcribe_dna);
    function_map
}
