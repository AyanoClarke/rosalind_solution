/// # Counting DNA Nucleotides
/// ## Backgrounds
/// adenine (A), cytosine (C), guanine (G), and thymine (T)
/// ## Problem
/// A string is simply an ordered collection of symbols selected from some alphabet and formed into a word;
/// the length of a string is the number of symbols that it contains.
/// An example of a length 21 DNA string (whose alphabet contains the symbols 'A', 'C', 'G', and 'T')
/// is "ATGCTTCAGAAAGGTCTTACG."
/// Given: A DNA string s of length at most 1000 nt.
/// Return: Four integers (separated by spaces) counting the respective number of times that the symbols 'A', 'C', 'G', and 'T' occur in s
/// ## Sample Dataset
/// ```
/// AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC
/// ```
/// ## Sample Output
/// ```
/// 20 12 17 21
/// ```


use std::fmt;

enum Nucleobases {
    A,
    // adenine
    G,
    // guanine
    T,
    // thymine
    C, // cytosine
}

#[derive(Debug, PartialEq, Eq)]
pub struct NucleobasesCounts {
    a: ANum,
    g: GNum,
    t: TNum,
    c: CNum,
}

struct NucleobasesCountsBuilder {
    a: ANum,
    g: GNum,
    t: TNum,
    c: CNum,
}

impl fmt::Display for NucleobasesCounts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "A: {}, C: {}, G: {}, T: {}",
               self.a, self.c, self.g, self.t)
    }
}

impl NucleobasesCounts {
    fn new() -> NucleobasesCountsBuilder {
        NucleobasesCountsBuilder {
            a: 0,
            g: 0,
            t: 0,
            c: 0,
        }
    }
}

impl NucleobasesCountsBuilder {
    fn a_plus(&mut self) -> &mut NucleobasesCountsBuilder {
        self.a += 1;
        self
    }
    fn g_plus(&mut self) -> &mut NucleobasesCountsBuilder {
        self.g += 1;
        self
    }
    fn t_plus(&mut self) -> &mut NucleobasesCountsBuilder {
        self.t += 1;
        self
    }
    fn c_plus(&mut self) -> &mut NucleobasesCountsBuilder {
        self.c += 1;
        self
    }

    fn build(&self) -> NucleobasesCounts {
        NucleobasesCounts {
            a: self.a,
            g: self.g,
            t: self.t,
            c: self.c,
        }
    }
}

type ANum = u32;
type GNum = u32;
type TNum = u32;
type CNum = u32;

fn check_nucleobase(s: char) -> Option<Nucleobases> {
    match s {
        'A' | 'a' => Some(Nucleobases::A),
        'G' | 'g' => Some(Nucleobases::G),
        'T' | 't' => Some(Nucleobases::T),
        'C' | 'c' => Some(Nucleobases::C),
        _ => None
    }
}

pub fn counting_DNA_nucleotides(s: &str) -> NucleobasesCounts {
    let mut counts = NucleobasesCounts::new();
    for char in s.chars() {
        match check_nucleobase(char) {
            Some(Nucleobases::A) => { counts.a_plus(); }
            Some(Nucleobases::G) => { counts.g_plus(); }
            Some(Nucleobases::T) => { counts.t_plus(); }
            Some(Nucleobases::C) => { counts.c_plus(); }
            _ => {}
        }
    }
    counts.build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let string = "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC";
        //assert_eq!((20,12,17,21), CountingDNANucleotides(string));
        assert_eq!(NucleobasesCounts { a: 0, g: 0, t: 0, c: 0 }, counting_DNA_nucleotides(string));
    }
}
