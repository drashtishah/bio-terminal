use std::collections::HashMap;

#[derive(Debug)]
pub enum FunctionResult {
    NucleotideCount(HashMap<char, usize>),
    TranscribedDNA(String),
    ReverseComplement(String),
    TranslatedRNA(String),
    ProteinMass(f64),
    HammingDistance(usize),
    MotifStartingLocations(Vec<usize>),
    GCContent(HashMap<String, f64>),
}

pub enum RosalindInputType {
    OneSequence(String),
    TwoSequence([String; 2]),
}

impl RosalindInputType {
    pub fn unwrap_sequence(&self) -> String {
        match self {
            RosalindInputType::OneSequence(t) => t.to_string(),
            _ => panic!("Called `RosalindInputType::unwrap_sequence()` on an invalid input"),
        }
    }
    pub fn unwrap_sequence_list(&self) -> [String; 2] {
        match self {
            RosalindInputType::TwoSequence(t) => t.clone(),
            _ => panic!("Called `RosalindInputType::unwrap_sequence_list()` on an invalid input"),
        }
    }
}

pub fn codon_table() -> HashMap<&'static str, char> {
    let table = HashMap::from([
        ("UUU", 'F'),
        ("UUC", 'F'),
        ("UUA", 'L'),
        ("UUG", 'L'),
        ("UCU", 'S'),
        ("UCC", 'S'),
        ("UCA", 'S'),
        ("UCG", 'S'),
        ("UAU", 'Y'),
        ("UAC", 'Y'),
        ("UAG", '*'),
        ("UGA", '*'),
        ("UGU", 'C'),
        ("UGC", 'C'),
        ("UGG", 'W'),
        ("GGG", 'G'),
        ("CUU", 'L'),
        ("CUC", 'L'),
        ("CUA", 'L'),
        ("CUG", 'L'),
        ("CCU", 'P'),
        ("CCC", 'P'),
        ("CCA", 'P'),
        ("CCG", 'P'),
        ("CAU", 'H'),
        ("CAC", 'H'),
        ("CAA", 'Q'),
        ("CAG", 'Q'),
        ("CGU", 'R'),
        ("CGC", 'R'),
        ("CGA", 'R'),
        ("CGG", 'R'),
        ("AUU", 'I'),
        ("AUC", 'I'),
        ("AUA", 'I'),
        ("ACG", 'T'),
        ("AUG", 'M'),
        ("ACU", 'T'),
        ("ACC", 'T'),
        ("ACA", 'T'),
        ("AAU", 'N'),
        ("AAC", 'N'),
        ("AAA", 'K'),
        ("AAG", 'K'),
        ("AGU", 'S'),
        ("AGC", 'S'),
        ("AGA", 'R'),
        ("AGG", 'R'),
        ("GUU", 'V'),
        ("GUC", 'V'),
        ("GUA", 'V'),
        ("GUG", 'V'),
        ("GCU", 'A'),
        ("GCC", 'A'),
        ("GCA", 'A'),
        ("GCG", 'A'),
        ("GAU", 'D'),
        ("GAC", 'D'),
        ("GAA", 'E'),
        ("GAG", 'E'),
        ("GGU", 'G'),
        ("GGC", 'G'),
        ("GGA", 'G'),
    ]);
    table
}

pub fn monoisotopic_mass_table() -> HashMap<char, f64> {
    let table = HashMap::from([
        ('A', 71.03711),
        ('C', 103.00919),
        ('D', 115.02694),
        ('E', 129.04259),
        ('F', 147.06841),
        ('G', 57.02146),
        ('H', 137.05891),
        ('I', 113.08406),
        ('K', 128.09496),
        ('L', 113.08406),
        ('M', 131.04049),
        ('N', 114.04293),
        ('P', 97.05276),
        ('Q', 128.05858),
        ('R', 156.10111),
        ('S', 87.03203),
        ('T', 101.04768),
        ('V', 99.06841),
        ('W', 186.07931),
        ('Y', 163.06333),
    ]);
    table
}

pub fn round_to_decimal_places(number: f64, places: i32) -> f64 {
    let factor = 10f64.powi(places);
    (number * factor).round() / factor
}