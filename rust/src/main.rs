use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::time::{Duration, Instant};

mod trie;

use trie::Trie;



fn main() -> io::Result<()> {
    // uncomment these lines if ../data/words.bin doesn't exist
    
    // let input_file_path = "../data/words_alpha.txt";
    // let data: Vec<String> = read_file_into_vector(input_file_path)?;

    let binary_file_path = "../data/words.bin";
    // write_vector_to_binary_file(&data, binary_file_path)?;

    let loaded_data: Vec<String> = read_binary_file_into_vector(binary_file_path)?;

    let mut t = Trie::new();

    for data in loaded_data {
        t.insert(&data);
    }


    t.search("test");

    Ok(())
}

fn read_file_into_vector(file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut data = Vec::new();
    for line in reader.lines() {
        data.push(line?);
    }

    Ok(data)
}

fn write_vector_to_binary_file(data: &Vec<String>, file_path: &str) -> io::Result<()> {
    let mut file = File::create(file_path)?;

    for line in data {
        file.write_all(line.as_bytes())?;
        file.write_all(b"\n")?; // Add newline after each line
    }

    Ok(())
}

fn read_binary_file_into_vector(file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut data = Vec::new();
    for line in reader.lines() {
        data.push(line?);
    }

    Ok(data)
}

