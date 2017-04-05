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

// use std::io;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
// use std::path::Path;
use std::collections::HashSet;
use std::env;


pub fn pattern_match(d: u8, template: &str, pattern: &str) -> bool {
    let k = pattern.len();
    let mut subtemplate;
    for i in 0..(template.len() - k + 1) {
        subtemplate = &template[i..i+k];
        let mut mismatches = 0;
        for j in 0..k {
            if subtemplate.as_bytes()[j] != pattern.as_bytes()[j] {
                mismatches += 1;
            }
        }
        if mismatches <= d {
            return true;
        }
    }

    false
}


fn motif_enumeration(dna: &[std::string::String], k: usize, d: u8) {
    println!("Running Motif Enumeration...");

    let mut matches = HashSet::new();
    let mut tried = HashSet::new();

    for line in dna {
        // go over k-mers in line
        'kmer: for i in 0..(line.len() - k + 1) {
            let kmer = &line[i..i+k];

            // if we've tried this kmer just continue
            if tried.contains(kmer) { continue 'kmer; }
            else { tried.insert(kmer); }

            // println!("{:?}", tried);

            // now do a d-pattern match across each dna string
            // if not found in any one, it's disqualified
            'dna: for line2 in dna {
                let qualified = pattern_match(d, line2, kmer);
                if !qualified {
                    // println!("Found valid kmer {:?}", kmer);
                    // try next kmer
                    continue 'kmer;
                }
            }
            // if we get here it means it's in all dna strings
            matches.insert(kmer);

        }
    }

    println!("{:?}", matches);
}

fn read_file(filename: &str) -> Vec<String> {
    let f = File::open(filename).unwrap();
    let buf = BufReader::new(&f);

    buf.lines().map(|l| l.expect("Could not parse line")).collect()
}


fn main() {
    println!("Hello, world!");

    let args: Vec<_> = env::args().collect();
    println!("Args provided: {:?}", args);

    let lines = read_file("./src/sample.txt");

    let line1: Vec<&str> = lines[0].split(" ").collect();
    let (k, d) = (line1[0].parse::<usize>().unwrap(), line1[1].parse::<u8>().unwrap());
    let dna = &lines[1..];

    motif_enumeration(dna, k, d);

}


//
// Testing
//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pattern_matching() {
        let mut matched = pattern_match(3, &"abcd".to_string(), &"efgh".to_string());
        assert!(matched==false);

        matched = pattern_match(3, &"abcd".to_string(), &"bcc".to_string());
        assert!(matched==true);
    }
}
