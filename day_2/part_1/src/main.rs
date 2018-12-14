use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() {
    let input_file = File::open("input.txt").unwrap();
    let file_reader = BufReader::new(input_file);
    
    let mut total_twos: u32 = 0;
    let mut total_threes: u32 = 0;
    for line in file_reader.lines().map(|x| x.unwrap()) {
        let (num_twos, num_threes) = scan_box(&line);
        total_twos += num_twos;
        total_threes += num_threes;
    }

    println!("Found {} strings with with at least one instance of two duplicate characters.", total_twos);
    println!("Found {} strings with at least one instance of three duplicate characters.", total_threes);
    println!("Resulting checksum: {}", total_threes * total_twos);
}

fn scan_box(line: &str) -> (u32, u32) {
    let mut char_count: HashMap<char, u32> = HashMap::new();
    for character in line.chars() {
        match char_count.get(&character) {
            Some(chr) => char_count.insert(character, chr + 1),
            None => char_count.insert(character, 1)
        };
    }

    let mut has_two: u32 = 0;
    let mut has_three: u32 = 0;
    for keypair in char_count.iter() {
        match *keypair.1 {
            2 => has_two = 1,
            3 => has_three = 1,
            _ => {}
        }
    }

    return (has_two, has_three);
}