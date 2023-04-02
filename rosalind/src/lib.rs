use std::collections::HashMap;

#[derive(Debug)]
pub enum FunctionResult {
    NucleotideCount(HashMap<char, usize>),
    TranscribedDna(String),
}

pub fn count_nucleotides(dna: &str) -> FunctionResult {
    let mut counts = HashMap::new();
    for c in dna.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    FunctionResult::NucleotideCount(counts)
}

pub fn transcribe_dna(dna: &str) -> FunctionResult {
    let rna = dna.replace("T", "U");
    FunctionResult::TranscribedDna(rna)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_nucleotides_test() {    
        let expected_values = [('A', 20), ('C', 12), ('G', 17), ('T', 21)];
        let expected_result: HashMap<char, usize> = expected_values.iter().cloned().collect();
        let actual_result = count_nucleotides("AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC");
    
        if let FunctionResult::NucleotideCount(actual_count) = actual_result {
            assert_eq!(actual_count, expected_result);
        } else {
            panic!("Unexpected function result");
        }
    }

    #[test]
    fn transcribe_dna_test() {
        let expected_result = "GAUGGAACUUGACUACGUAAAUU";
        let actual_result = transcribe_dna("GATGGAACTTGACTACGTAAATT");
    
        if let FunctionResult::TranscribedDna(actual_rna) = actual_result {
            assert_eq!(actual_rna, expected_result);
        } else {
            panic!("Unexpected function result");
        }
    }
}
