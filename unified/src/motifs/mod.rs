
use std::u32::MAX;

use basic;

// MedianString(Dna, k)
//     distance ← ∞
//     for each k-mer Pattern from AA…AA to TT…TT
//         if distance > d(Pattern, Dna)
//              distance ← d(Pattern, Dna)
//              Median ← Pattern
//     return Median

// pub fn median_string(k: u32, dna: Vec<&str>) -> String {
//   let mut distance: u32 = MAX;
//
//   let mut running_d_collection;
//   let mut median: String;
//   for kmer in basic::all_kmers(k as usize) {
//     running_d_collection = distance_collection(kmer, dna);
//     if distance > running_d_collection {
//         distance = running_d_collection;
//         median = kmer;
//     }
//   }
//
//   median
// }
//
// fn distance_collection(kmer: &str, dna: Vec<&str>) -> u32 {
//   dna.iter().fold(0u32, |sum, motif| sum + distance_motif(kmer, motif))
// }
//
// // find the min distance between a rolling kmer on a motif
// fn distance_motif(kmer: &str, motif: &str) -> u32 {
//     unimplemented!();
// }
