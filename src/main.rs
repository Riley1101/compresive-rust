#![allow(unused_imports)]
use std::fmt::Display;

use compresive_rust::arrays::contains_duplicates::valid_anagram;

#[derive(Debug)]
struct ParsedFile {
    contents: Vec<usize>,
    size: usize,
}

impl ParsedFile {
    fn new() -> Self {
        Self {
            contents: Vec::new(),
            size: 0,
        }
    }

    fn push(&mut self, num: usize) {
        self.contents.push(num);
        self.size += 1;
    }

    fn multiply(&mut self) {
        let mut count = 0;
        for i in self.contents.iter_mut() {
            count += 1;
            self.contents[count] = *i + 12;
        }
    }
}

impl Display for ParsedFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:#?}", self)
    }
}

fn main() {
    let mut p = ParsedFile::new();
    p.push(12);
    p.push(14);
    p.multiply();
    println!("{p}");
}
