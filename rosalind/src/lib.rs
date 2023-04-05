pub mod utils;

use std::collections::HashMap;
use utils::{FunctionResult, 
            codon_table, 
            monoisotopic_mass_table, 
            round_to_decimal_places};

pub fn count_nucleotides(dna: &str) -> FunctionResult {
    let mut counts = HashMap::new();
    for c in dna.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    FunctionResult::NucleotideCount(counts)
}

pub fn translate_rna(rna: &str) -> FunctionResult {
    let protein = rna.as_bytes()
                .chunks(3)
                .filter_map(|chunk| {
                    let codon = std::str::from_utf8(chunk).unwrap();
                    match codon_table().get(codon) {
                        Some(&amino_acid) if amino_acid != '*' => Some(amino_acid),
                        _ => None,
                    }
                })
                .collect();
    FunctionResult::TranslatedRNA(protein)
}

pub fn transcribe_dna(dna: &str) -> FunctionResult {
    let rna = dna.replace("T", "U");
    FunctionResult::TranscribedDNA(rna)
}

pub fn protein_mass(protein: &str) -> FunctionResult {
    let mut mass = protein.chars()
                            .map(|x| monoisotopic_mass_table()[&x])
                            .sum::<f64>();
    mass = round_to_decimal_places(mass, 3);
    FunctionResult::ProteinMass(mass)
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

    #[test]
    fn translate_rna_test() {
        let expected_result = "MAMAPRTEINSTRING";
        let actual_result = translate_rna("AUGGCCAUGGCGCCCAGAACUGAGAUCAAUAGUACCCGUAUUAACGGGUGA");
    
        if let FunctionResult::TranslatedRNA(actual_protein) = actual_result {
            assert_eq!(actual_protein, expected_result);
        } else {
            panic!("Unexpected function result");
        }
    }

    #[test]
    fn protein_mass_test() {
        let expected_result = 821.392;
        let actual_result = protein_mass("SKADYEK");
    
        if let FunctionResult::ProteinMass(actual_mass) = actual_result {
            assert_eq!(actual_mass, expected_result);
        } else {
            panic!("Unexpected function result");
        }
    }
}
