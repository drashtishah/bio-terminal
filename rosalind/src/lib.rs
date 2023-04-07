pub mod utils;

use std::collections::HashMap;
use utils::{codon_table, monoisotopic_mass_table, round_to_decimal_places, FunctionResult, RosalindInputType};

/// Counts the occurrences of each nucleotide in a DNA string.
/// Returns a FunctionResult::NucleotideCount variant containing a HashMap of nucleotide counts.           
pub fn count_nucleotides(input: RosalindInputType) -> FunctionResult {
    let dna = input.unwrap_sequence();
    let mut counts = HashMap::new();
    for c in dna.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    FunctionResult::NucleotideCount(counts)
}

/// Calculate the Hamming distance between two DNA strings.
/// The Hamming distance is the number of corresponding symbols that differ in two given DNA strings.
pub fn hamming_distance(input: RosalindInputType) -> FunctionResult {
    let [s1, s2] = input.unwrap_sequence_list();
    let distance = s1.chars()
                            .zip(s2.chars())
                            .filter(|(c1, c2)| c1 != c2)
                            .count();
    FunctionResult::HammingDistance(distance)
}

/// Translates an RNA string into a protein string.
/// Returns a FunctionResult::TranslatedRNA variant containing the translated protein string.
pub fn translate_rna(input: RosalindInputType) -> FunctionResult {
    let rna = input.unwrap_sequence();
    let protein = rna
        .as_bytes()
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

/// Transcribes a DNA string into an RNA string.
/// Returns a FunctionResult::TranscribedDNA variant containing the transcribed RNA string.
pub fn transcribe_dna(input: RosalindInputType) -> FunctionResult {
    let dna = input.unwrap_sequence();
    let rna = dna.replace("T", "U");
    FunctionResult::TranscribedDNA(rna)
}

/// Calculates the mass of a protein string.
/// Returns a FunctionResult::ProteinMass variant containing the protein mass as a floating-point number.
pub fn protein_mass(input: RosalindInputType) -> FunctionResult {
    let protein = input.unwrap_sequence();
    let mut mass = protein
        .chars()
        .map(|x| monoisotopic_mass_table()[&x])
        .sum::<f64>();
    mass = round_to_decimal_places(mass, 3);
    FunctionResult::ProteinMass(mass)
}

/// Computes the reverse complement of a DNA string.
/// Returns a FunctionResult::ReverseComplement variant containing the reverse complement of the input DNA string.
pub fn reverse_complement(input: RosalindInputType) -> FunctionResult {
    let dna = input.unwrap_sequence();
    let reverse_complement = dna
        .chars()
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

// Test module containing tests for each function
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_nucleotides_test() {
        let expected_values = [('A', 20), ('C', 12), ('G', 17), ('T', 21)];
        let expected_result: HashMap<char, usize> = expected_values.iter().cloned().collect();
        let actual_result = count_nucleotides(RosalindInputType::OneSequence("AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC".to_string()));
        if let FunctionResult::NucleotideCount(actual_count) = actual_result {
            assert_eq!(actual_count, expected_result);
        } else {
            panic!("Unexpected function result");
        }
    }

    #[test]
    fn transcribe_dna_test() {
        let expected_result = "GAUGGAACUUGACUACGUAAAUU";
        let actual_result = transcribe_dna(RosalindInputType::OneSequence("GATGGAACTTGACTACGTAAATT".to_string()));

        if let FunctionResult::TranscribedDNA(actual_rna) = actual_result {
            assert_eq!(actual_rna, expected_result);
        } else {
            panic!("Unexpected function result");
        }
    }

    #[test]
    fn reverse_complement_test() {
        let expected_result = "ACCGGGTTTT";
        let actual_result = reverse_complement(RosalindInputType::OneSequence("AAAACCCGGT".to_string()));

        if let FunctionResult::ReverseComplement(actual_dna) = actual_result {
            assert_eq!(actual_dna, expected_result);
        } else {
            panic!("Unexpected function result");
        }
    }

    #[test]
    fn translate_rna_test() {
        let expected_result = "MAMAPRTEINSTRING";
        let actual_result = translate_rna(RosalindInputType::OneSequence("AUGGCCAUGGCGCCCAGAACUGAGAUCAAUAGUACCCGUAUUAACGGGUGA".to_string()));

        if let FunctionResult::TranslatedRNA(actual_protein) = actual_result {
            assert_eq!(actual_protein, expected_result);
        } else {
            panic!("Unexpected function result");
        }
    }

    #[test]
    fn protein_mass_test() {
        let expected_result = 821.392;
        let actual_result = protein_mass(RosalindInputType::OneSequence("SKADYEK".to_string()));

        if let FunctionResult::ProteinMass(actual_mass) = actual_result {
            assert_eq!(actual_mass, expected_result);
        } else {
            panic!("Unexpected function result");
        }
    }

    #[test]
    fn hamming_distance_test() {
        let expected_result = 7;
        let actual_result = hamming_distance(RosalindInputType::TwoSequence(["GAGCCTACTAACGGGAT".to_string(), "CATCGTAATGACGGCCT".to_string()]));

        if let FunctionResult::HammingDistance(actual_distance) = actual_result {
            assert_eq!(actual_distance, expected_result);
        } else {
            panic!("Unexpected function result");
        }
    }
}
