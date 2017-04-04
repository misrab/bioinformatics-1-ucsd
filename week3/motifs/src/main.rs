// Input: Integers k and d, followed by a collection of strings Dna.
//  Output: All (k, d)-motifs in Dna.
//
// MotifEnumeration(Dna, k, d)
//     Patterns ← an empty set
//     for each k-mer Pattern in Dna
//         for each k-mer Pattern’ differing from Pattern by at most d mismatches
//             if Pattern' appears in each string from Dna with at most d ﻿mismatches
//                 add Pattern' to Patterns
//     remove duplicates from Patterns
//     return Patterns

use std::io;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::path::Path;

fn MotifEnumeration(dna: &[std::string::String], k: usize, d: u8) {
    println!("Running Motif Enumeration...");

    for line in dna {
        // go over k-mers in line
        for i in 0..(line.len() - k + 1) {
            let kmer = &line[i..i+k];
            println!("{:?}", kmer);
        }
    }
}

fn ReadFile(filename: &str) -> Vec<String> {
    let f = File::open(filename).unwrap();
    let buf = BufReader::new(&f);

    buf.lines().map(|l| l.expect("Could not parse line")).collect()
}


fn main() {
    println!("Hello, world!");

    let lines = ReadFile("./src/sample.txt");

    let line1: Vec<&str> = lines[0].split(" ").collect();
    let (k, d) = (line1[0].parse::<usize>().unwrap(), line1[1].parse::<u8>().unwrap());
    let dna = &lines[1..];

    MotifEnumeration(dna, k, d);

}
