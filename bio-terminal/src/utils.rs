use std::collections::HashMap;
use rosalind::count_nucleotides;

pub fn print_challenges() {
    println!("{}", "Select the problem you want to solve.");
    println!("{}", "1. Counting DNA Nucleotides");
}

pub fn function_map() -> HashMap<i32, fn(&str) -> HashMap<char, usize>> {
    let mut function_map: HashMap<i32, fn(&str) -> HashMap<char, usize>> = HashMap::new();
    function_map.insert(1, count_nucleotides);
    function_map
}

pub fn run_solution(num: i32, sequence: &str) {
    
    
}