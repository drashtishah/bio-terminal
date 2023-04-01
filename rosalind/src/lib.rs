use std::collections::HashMap;

pub fn count_nucleotides(dna: &str) -> HashMap<char, usize> {
    let mut counts = HashMap::new();
    for c in dna.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    counts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_nucleotides_test() {
        let expected_values = [('A', 20), ('C', 12), ('G', 17), ('T', 21)];
        let expected_result = HashMap::from(expected_values);
        let actual_result = count_nucleotides("AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC");
        assert_eq!(actual_result, expected_result);
    }
}
