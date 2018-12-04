use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() {    
    let mut seen_freqencies: HashMap<i64, bool> = HashMap::new();
    let mut found_freqency: bool = false;

    let mut running_total: i64 = 0;

    while !found_freqency {
        let input_file = File::open("input.txt").unwrap();
        let file_reader = BufReader::new(input_file);
        for line in file_reader.lines().map(|x| x.unwrap()) {
            let mut line_reader = line.chars();
            let first_char = line_reader.next();
            let string_value = line_reader.as_str();
            let numeric_value = match first_char.unwrap() {
                '+' => string_value.parse::<i64>().unwrap(),
                '-' => string_value.parse::<i64>().unwrap() * -1,
                _ => panic!("Invalid input data oh noes"),
            };        
            running_total = adjust_freqency(running_total, numeric_value);
            if seen_freqencies.contains_key(&running_total) {
                found_freqency = true;
                break;
            }
            seen_freqencies.insert(running_total, true);
        }
    }

    println!("Number of seen freqencies: {0}", seen_freqencies.len());
    println!("Final value: {0}", running_total);
}

fn adjust_freqency(prev_value: i64, adjustment: i64) -> i64 {
    prev_value + adjustment
}
