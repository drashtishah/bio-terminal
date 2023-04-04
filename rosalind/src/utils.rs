use std::collections::HashMap;

#[derive(Debug)]
pub enum FunctionResult {
    NucleotideCount(HashMap<char, usize>),
    TranscribedDNA(String),
    ReverseComplement(String),
    TranslatedRNA(String),
}

pub fn codon_table() -> HashMap<&'static str, char> {
    let table = HashMap::from([
        ("UUU", 'F'), ("UUC", 'F'), ("UUA", 'L'), ("UUG", 'L'),
        ("UCU", 'S'), ("UCC", 'S'), ("UCA", 'S'), ("UCG", 'S'),
        ("UAU", 'Y'), ("UAC", 'Y'), ("UAG", '*'), ("UGA", '*'),
        ("UGU", 'C'), ("UGC", 'C'), ("UGG", 'W'), ("GGG", 'G'),
        ("CUU", 'L'), ("CUC", 'L'), ("CUA", 'L'), ("CUG", 'L'),
        ("CCU", 'P'), ("CCC", 'P'), ("CCA", 'P'), ("CCG", 'P'),
        ("CAU", 'H'), ("CAC", 'H'), ("CAA", 'Q'), ("CAG", 'Q'),
        ("CGU", 'R'), ("CGC", 'R'), ("CGA", 'R'), ("CGG", 'R'),
        ("AUU", 'I'), ("AUC", 'I'), ("AUA", 'I'), ("ACG", 'T'),
        ("AUG", 'M'), ("ACU", 'T'), ("ACC", 'T'), ("ACA", 'T'), 
        ("AAU", 'N'), ("AAC", 'N'), ("AAA", 'K'), ("AAG", 'K'),
        ("AGU", 'S'), ("AGC", 'S'), ("AGA", 'R'), ("AGG", 'R'),
        ("GUU", 'V'), ("GUC", 'V'), ("GUA", 'V'), ("GUG", 'V'),
        ("GCU", 'A'), ("GCC", 'A'), ("GCA", 'A'), ("GCG", 'A'),
        ("GAU", 'D'), ("GAC", 'D'), ("GAA", 'E'), ("GAG", 'E'),
        ("GGU", 'G'), ("GGC", 'G'), ("GGA", 'G'), 
    ]);
    table
}
