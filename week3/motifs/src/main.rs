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

// static NUCLEOTIDES: &'static [char] = &['A', 'C', 'T', 'G'];
static NUCLEOTIDES: [char; 4] = ['A', 'C', 'T', 'G'];

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

    if dna.len() == 0 { return; }


    // let mut matches = HashSet::new();
    // let mut tried = HashSet::new();

    // first get all dneighbours of kmers in first string
    print!("{:?}", dna[0]);

    // let first = &dna[0];
    // let mut candidates: Vec<&str>;
    // 'kmer: for i in 0..(first.len() - k + 1) {
    //     let kmer = &first[i..i+k];
    //     candidates = get_kmer_dneighbours(k, d, kmer);
    // }

    // for line in dna {
    //     // go over k-mers in line
    //     'kmer: for i in 0..(line.len() - k + 1) {
    //         let kmer = &line[i..i+k];
    //
    //         let dneighbours = get_kmer_dneighbours(k, d, kmer);
    //
    //         // if we've tried this kmer just continue
    //         if tried.contains(kmer) { continue 'kmer; }
    //         else { tried.insert(kmer); }
    //
    //         // println!("{:?}", tried);
    //
    //         // now do a d-pattern match across each dna string
    //         // if not found in any one, it's disqualified
    //         'dna: for line2 in dna {
    //             let qualified = pattern_match(d, line2, kmer);
    //             if !qualified {
    //                 // println!("Found valid kmer {:?}", kmer);
    //                 // try next kmer
    //                 continue 'kmer;
    //             }
    //         }
    //         // if we get here it means it's in all dna strings
    //         matches.insert(kmer);
    //
    //     }
    // }

    // println!("{:?}", matches);
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
    let filename = &args[1];

    let filepath = format!("{}{}", "./src/", filename);
    let lines = read_file(filepath.as_str());

    let line1: Vec<&str> = lines[0].split(" ").collect();
    let (k, d) = (line1[0].parse::<usize>().unwrap(), line1[1].parse::<u8>().unwrap());
    let dna = &lines[1..];

    motif_enumeration(dna, k, d);

}


//
// Neighbors(Pattern, d)
//     if d = 0
//         return {Pattern}
//     if |Pattern| = 1
//         return {A, C, G, T}
//     Neighborhood ← an empty set
//     SuffixNeighbors ← Neighbors(Suffix(Pattern), d)
//     for each string Text from SuffixNeighbors
//         if HammingDistance(Suffix(Pattern), Text) < d
//             for each nucleotide x
//                 add x • Text to Neighborhood
//         else
//             add FirstSymbol(Pattern) • Text to Neighborhood
//     return Neighborhood
// maybe a faster way to do this, but it's UTF-8 so indexing doens't work
// i.e. it's byte indexing...
fn suffix(s: &str) -> String {
    // &s[1..s.len()-1]
    s.chars().skip(1).take(s.len()-1).collect()
}

fn hamming(a: &str, b: &str) -> u8 {
    let mut mismatches = 0;
    for i in 0..a.len() {
        if a.chars().nth(i) != b.chars().nth(i) {
            mismatches += 1;
        }
    }

    mismatches
}

// for a given kmer, get all dneighbours
fn get_kmer_dneighbours(k: usize, d: u8, kmer: &str) -> HashSet<String> {
    let kmer_string = kmer.to_string();
    // result.insert(kmer);

    // if no error allowed, return set with string itself
    if d==0 {
        return [ kmer.to_string() ].iter().cloned().collect();
    }
    // if singleton, return singletons
    if kmer_string.len() == 1 {
        return ["G".to_string(), "A".to_string(), "T".to_string(), "C".to_string()].iter().cloned().collect();
    }

    let mut neighbourhood = HashSet::new();
    let suff = &suffix(kmer);
    let suffix_neighbours = get_kmer_dneighbours(k-1, d, suff);

    let mut n: String;
    for text in suffix_neighbours {
        if hamming(suff, &text) < d {
            for x in NUCLEOTIDES.iter() {
                n = x.to_string();
                n.push_str(&text);

                neighbourhood.insert(n);
            }
        } else {
            // let novel = kmer.chars().nth(0).to_string().push_str(text);
            n = kmer.chars().nth(0).unwrap().to_string();
            n.push_str(&text);
            neighbourhood.insert(n);
        }
    }


    neighbourhood
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

    #[test]
    fn find_d_neighbourhood() {
        let pattern = "GA"; //.to_string();

        let k = pattern.len();
        let mut neighbours = get_kmer_dneighbours(k, 0, pattern);
        assert!(neighbours.len() == 1);

        neighbours = get_kmer_dneighbours(k, 1, pattern);
        let mut benchmark: HashSet<String> = HashSet::new();
        benchmark.insert("GA".to_string());
        benchmark.insert("GT".to_string());
        benchmark.insert("GC".to_string());
        benchmark.insert("GG".to_string());

        benchmark.insert("TA".to_string());
        benchmark.insert("CA".to_string());
        benchmark.insert("GA".to_string());

        println!("{:?}\n{:?}", neighbours, benchmark);
        assert!(neighbours==benchmark);
    }
}
