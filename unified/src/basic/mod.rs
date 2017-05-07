use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::env;



static NUM_NUCLEOTIDES: u32 = 4;
static NUCLEOTIDES: [char; 4] = ['A', 'C', 'T', 'G'];


// an iterator for k-mers
pub struct AllKmers {
  k: usize,
  curr_index: u32,
  max_kmers: u32,
}
impl Iterator for AllKmers {
  type Item = String;

  fn next(&mut self) -> Option<String> {
    if self.curr_index > self.max_kmers { return None; }

    let kmer = kmer_from_index(self.curr_index);

    self.curr_index += 1;

    Some(kmer)
  }
}
pub fn all_kmers(k: usize) -> AllKmers {
  let max = NUM_NUCLEOTIDES.pow(k as u32) - 1;
  AllKmers { k: k, curr_index: 0, max_kmers: max }
}
// We let A:0,C:1,T:2,G:3
// We then let "ACT" = 0*4^2 + 1*4^1 + 2*4^0 as our mapping
fn kmer_from_index(index: u32) -> String {
  let mut kmer = String::from("");

  

  kmer
}

// def PatternToIndex(k, pattern):
//     # print "finding pattern index for " + pattern + " of length " + str(len(pattern))
//     index = 0
//     for i in xrange(k):
//         index += 4**(k-i-1) * NucleotideToValue(pattern[i])
//
//
//     return index
//
// def IndexToPattern(k, index):
//     result = ''
//
//     for i in xrange(k):
//         base = 4**(k-i-1)
//         # how many of these are there
//         # e.g. how many 4^6's
//         count = index / base
//         nucleotide = ValueToNucleotide(count)
//         result += nucleotide
//
//         # subtract the value found
//         index -= count*base
//
//     return result


pub fn hamming(a: &str, b: &str) -> i32 {
  let mut mismatches = 0;

  let a_chars: Vec<char> = a.chars().collect();
  let b_chars: Vec<char> = b.chars().collect();

  for i in 0..a_chars.len() as usize {
    if a_chars[i] != b_chars[i] { mismatches += 1; }
  }

  mismatches
}


// take a filename argument to the program and
// returns the file
// e.g. lines = get_file_argument()
// let line1: Vec<&str> = lines[0].split(" ").collect();
pub fn get_file_lines() -> Vec<String> {
    let args: Vec<_> = env::args().collect();
    println!("Args provided: {:?}", args);

    // let filename = &args[1];
    // let filepath = format!("{}{}", "./", filename);

    let filepath = &args[1];

    read_file(filepath.as_str())

    // let line1: Vec<&str> = lines[0].split(" ").collect();
    // let (k, d) = (line1[0].parse::<usize>().unwrap(), line1[1].parse::<u8>().unwrap());
    // let dna = &lines[1..];
}



//
// Internal
//

fn read_file(filename: &str) -> Vec<String> {
    let f = File::open(filename).unwrap();
    let buf = BufReader::new(&f);

    buf.lines().map(|l| l.expect("Could not parse line")).collect()
}





#[cfg(test)]



#[test]
fn test_all_kmers() {
    let three_mers = all_kmers(3);

    for i in three_mers.take(5) {
        println!("{}", i);
    }
}

#[test]
#[ignore]
fn test_hamming() {
  let mut ham = hamming("xyz","pqr");
  assert!(ham == 3);

  ham = hamming("xyz", "xyz");
  assert!(ham == 0);
}
