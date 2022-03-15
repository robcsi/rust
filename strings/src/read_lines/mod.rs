use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn readlines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    read_lines(filename)
}

struct Matrix {
    elements: Vec<i32>
}

impl Iterator for Matrix {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.elements.iter().next() {
            Some(val) => Some(*val),
            None => None
        }
    }
}
