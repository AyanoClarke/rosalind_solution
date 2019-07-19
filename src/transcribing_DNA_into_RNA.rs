/// # Problem
/// An RNA string is a string formed from the alphabet containing 'A', 'C', 'G', and 'U'.
/// Given a DNA string t
/// corresponding to a coding strand, its transcribed RNA string u is formed by
/// replacing all occurrences of 'T' in t with 'U' in u.
/// Given: A DNA string t
/// having length at most 1000 nt.
/// Return: The transcribed RNA string of t.
/// ## Sample Dataset
/// ```
/// GATGGAACTTGACTACGTAAATT
/// ```
/// ## Sample Output
/// ```
/// GAUGGAACUUGACUACGUAAAUU
/// ```

use std::fmt;

#[derive(Copy, Clone, PartialEq)]
enum Nucleobase {
    A,
    T,
    C,
    G,
    U,
}

fn char_to_nucleobase(c: char) -> Option<Nucleobase> {
    match c {
        'A' => { Some(Nucleobase::A) }
        'T' => { Some(Nucleobase::T) }
        'C' => { Some(Nucleobase::C) }
        'G' => { Some(Nucleobase::G) }
        'U' => { Some(Nucleobase::U) }
        _ => { None }
    }
}

impl fmt::Display for Nucleobase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
               match self {
                   Nucleobase::A => { "A" }
                   Nucleobase::T => { "T" }
                   Nucleobase::C => { "C" }
                   Nucleobase::G => { "G" }
                   Nucleobase::U => { "U" }
               }
        )
    }
}


pub struct DNA {
    nucleobases: Vec<Nucleobase>,
}

impl From<&str> for DNA {
    #[inline]
    fn from(s: &str) -> DNA {
        let mut dna = DNA::new();
        let chars = s.chars();
        for char in chars {
            if let Some(n) = char_to_nucleobase(char) {
                dna.add_nucleobase(n);
            }
        }
        dna
    }
}

#[derive(PartialEq)]
pub struct RNA {
    nucleobases: Vec<Nucleobase>,
}

impl fmt::Display for RNA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.nucleobases.iter().fold(Ok(()), |result, nucleotide| {
            result.and_then(|_| write!(f, "{}", nucleotide))
        })
    }
}

impl fmt::Debug for RNA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.nucleobases.iter().fold(Ok(()), |result, nucleotide| {
            result.and_then(|_| write!(f, "{}", nucleotide))
        })
    }
}

impl RNA {
    fn new() -> RNA {
        RNA { nucleobases: Vec::new() }
    }
    fn add_nucleobase(&mut self, nucleobase: Nucleobase) {
        self.nucleobases.push(nucleobase);
    }
}

impl From<&str> for RNA {
    #[inline]
    fn from(s: &str) -> RNA {
        let mut rna = RNA::new();
        let chars = s.chars();
        for char in chars {
            if let Some(n) = char_to_nucleobase(char) {
                rna.add_nucleobase(n);
            }
        }
        rna
    }
}

impl DNA {
    fn new() -> DNA {
        DNA { nucleobases: Vec::new() }
    }

    fn add_nucleobase(&mut self, nucleobase: Nucleobase) -> &mut DNA {
        self.nucleobases.push(nucleobase);
        self
    }

    pub fn transcribing(&self) -> RNA {
        let mut rna = RNA::new();
        self.nucleobases.iter().for_each(
            |n|
                rna.add_nucleobase(match n {
                    Nucleobase::A => { Nucleobase::A }
                    Nucleobase::C => { Nucleobase::C }
                    Nucleobase::G => { Nucleobase::G }
                    _ => { Nucleobase::U }
                })
        );
        rna
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(RNA::from("GAUGGAACUUGACUACGUAAAUU"), DNA::from("GATGGAACTTGACTACGTAAATT").transcribing());
    }
}


