use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input_file = File::open("input.txt").unwrap();
    let file_reader = BufReader::new(input_file);

    let mut running_total: i64 = 0;
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
    }

    println!("Final value: {0}", running_total);
}

fn adjust_freqency(prev_value: i64, adjustment: i64) -> i64 {
    prev_value + adjustment
}
