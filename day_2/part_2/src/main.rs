use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() {
    let input_file = File::open("input.txt").unwrap();
    let file_reader = BufReader::new(input_file);
    
    let all_lines: Vec<String> = file_reader.lines().map(|x| x.unwrap()).collect();
    let (box1, box2) = find_correct_box_ids(all_lines);
    let common_string = get_common_chars_string(&box1, &box2);

    println!("The two box IDs were: {}, {}. Their common characters are: {}", box1, box2, common_string);
}

fn find_correct_box_ids(all_lines: Vec<String>) -> (String, String) {
    for line_a in all_lines.iter() {
        for line_b in all_lines.iter() {
            let diffcount = get_diffcount(line_a, line_b);
            if diffcount == 1 {
                return (line_a.to_string(), line_b.to_string()); 
            }
        }
    }
    return ("".to_string(), "".to_string());
}

fn get_diffcount(str_a: &str, str_b: &str) -> u32 {
    let mut diffcount = 0u32;
    for (chr_a, chr_b) in str_a.chars().zip(str_b.chars()) {
        if chr_a != chr_b {
            diffcount += 1;
        }
    }

    diffcount
}

fn get_common_chars_string(str_a: &str, str_b: &str) -> String {
    let mut common: String = String::new();
    for (chr_a, chr_b) in str_a.chars().zip(str_b.chars()) {
        if chr_a == chr_b {
            common.push(chr_a);
        }
    }

    common
}