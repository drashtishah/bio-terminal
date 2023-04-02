use std::collections::HashMap;

#[derive(Debug)]
pub enum FunctionResult {
    NucleotideCount(HashMap<char, usize>),
    TranscribedDNA(String),
    ReverseComplement(String),
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
    FunctionResult::TranscribedDNA(rna)
}

pub fn reverse_complement(dna: &str) -> FunctionResult {
    let reverse_complement = dna.chars()
                                .rev()
                                .map(|c| match c {
                                    'A' => 'T',
                                    'T' => 'A',
                                    'C' => 'G',
                                    'G' => 'C',
                                    _ => panic!("Invalid character in DNA sequence"),
                                })
                                .collect();
    FunctionResult::ReverseComplement(reverse_complement)
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
    
        if let FunctionResult::TranscribedDNA(actual_rna) = actual_result {
            assert_eq!(actual_rna, expected_result);
        } else {
            panic!("Unexpected function result");
        }
    }

    #[test]
    fn reverse_complement_test() {
        let expected_result = "ACCGGGTTTT";
        let actual_result = reverse_complement("AAAACCCGGT");
    
        if let FunctionResult::ReverseComplement(actual_dna) = actual_result {
            assert_eq!(actual_dna, expected_result);
        } else {
            panic!("Unexpected function result");
        }
    }
}
