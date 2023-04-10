use polars::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn fasta_to_dataframe(file_path: &str) -> Result<DataFrame, Box<dyn std::error::Error>> {
    // Your implementation here> {
    // Open the file and create a buffered reader
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Initialize vectors to store IDs and sequences
    let mut ids = Vec::new();
    let mut sequences = Vec::new();
    let mut current_sequence = String::new();

    // Read the FASTA file line by line
    for line in reader.lines() {
        let line = line?;

        // Check if the line starts with '>'
        if line.starts_with('>') {
            // Store the previous sequence and ID
            if !current_sequence.is_empty() {
                sequences.push(current_sequence);
                current_sequence = String::new();
            }
            ids.push(line[1..].to_string());
        } else {
            // If the line does not start with '>', add it to the current sequence
            current_sequence.push_str(&line);
        }
    }
    // Add the last sequence to the `sequences` vector
    if !current_sequence.is_empty() {
        sequences.push(current_sequence);
    }

    // Create a DataFrame from the collected IDs and sequences
    let id_series = Series::new("ID", ids);
    let sequence_series = Series::new("Sequence", sequences);

    let dataframe = DataFrame::new(vec![id_series, sequence_series])?;

    // Return the DataFrame
    Ok(dataframe)
}
