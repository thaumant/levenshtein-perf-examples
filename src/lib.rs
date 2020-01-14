mod utils;
mod version0;
mod version1;
mod version2;
mod version3;
mod version4;
mod version5;

pub use version0::levenshtein0;
pub use version1::levenshtein1;
pub use version2::levenshtein2;
pub use version3::levenshtein3;
pub use version4::levenshtein4;
pub use version5::levenshtein5;


#[cfg(test)]
mod tests {
    use super::{
        levenshtein0,
        levenshtein1,
        levenshtein2,
        levenshtein3,
        levenshtein4,
        levenshtein5,
    };

    #[test]
    fn test_godel() {
        let s = "Gödel";
        let b = s.as_bytes();
        dbg!(b);
    }

    #[test]
    fn check_levenshtein() {
        let sample = [ 
            (0, "",          ""),
            (0, "foo",       "foo"),
            (3, "foo",       ""),
            (3, "ca",        "abc"),
            (3, "a tc",      "a cat"),
            (4, "a cat",     "an abct"),
            (2, "crate",     "trace"),
            (2, "captain",   "ptain"),
            (2, "dwayne",    "duane"),
            (2, "martha",    "marhta"),
            (3, "kitten",    "sitting"),
            (6, "mailbox",   "boxmail"),
            (3, "mailbox",   "alimbox"),
            (4, "dixon",     "dicksonx"),
            (2, "jellyfish", "smellyfish"),
        ];
        for &(d, s1, s2) in &sample {
            assert_eq!(levenshtein0(s1, s2), d);
            assert_eq!(levenshtein0(s2, s1), d);
            
            assert_eq!(levenshtein1(s1, s2), d);
            assert_eq!(levenshtein1(s2, s1), d);

            assert_eq!(levenshtein2(s1, s2), d);
            assert_eq!(levenshtein2(s2, s1), d);

            assert_eq!(levenshtein3(s1, s2), d);
            assert_eq!(levenshtein3(s2, s1), d);

            assert_eq!(levenshtein4(s1, s2), d);
            assert_eq!(levenshtein4(s2, s1), d);

            assert_eq!(levenshtein5(s1, s2), d);
            assert_eq!(levenshtein5(s2, s1), d);
        }
    }
}
