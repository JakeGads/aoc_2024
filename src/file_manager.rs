use std::fs::File;
use std::io::{BufRead, BufReader};
pub fn read(path: String, delimiter: Option<char>) -> Vec<String> {
    if delimiter.is_some() {
        return read_delimited(path, delimiter.unwrap())
    }
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    lines
}

fn read_delimited(path: String, delimiter: char) -> Vec<String> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.split(delimiter as u8).map(|line| String::from_utf8(line.unwrap()).unwrap()).collect();
    lines
}