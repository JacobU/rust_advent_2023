use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::io::BufReader;

pub fn parse_lines_of_strings(filename: String) -> Vec<String> {    
    let reader: BufReader<File> = BufReader::new(File::open(filename).unwrap());
    let lines = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();

    return lines;
}

pub fn parse_lines_of_strings_with_delim(filename: String, delim: String) -> Vec<Vec<String>> {
    let lines: Vec<String> = parse_lines_of_strings(filename);
    let mut new_lines: Vec<Vec<String>> = vec![]; 

    for line in lines {
        let temp_line: Vec<String> = line.split(&delim).map(|x| x.to_string()).collect();
        new_lines.push(temp_line);
    }

    return new_lines;
}

pub fn parse_number_of_lines_of_strings_together(filename: String, size_of_group: i32) -> Vec<Vec<String>> {
    let mut lines: Vec<Vec<String>> = vec![];

    let mut count = 1;
    let mut templines: Vec<String> = vec![];
    for line in std::fs::read_to_string(filename).unwrap().lines() {
        templines.push(line.to_string().clone());
        if count % size_of_group == 0 {
            lines.push(templines.clone());
            templines.clear();
        }
        count += 1;
    }
    lines
}

pub fn get_nums_from_string_instruction(line: String) -> Vec<i32> {
    return line.split_ascii_whitespace().filter_map(|x| x.parse().ok()).collect();
}