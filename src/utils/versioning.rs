// use std::cmp::Ordering;

use regex::Regex;

pub trait DataVersion {
    fn pattern(&self) -> regex::Regex;

    // fn compare(a: &str, b: &str) -> Ordering {
    //     if a > b {
    //         Ordering::Greater
    //     } else {
    //         Ordering::Less
    //     }
    // }
}

pub struct WildcardVersion {}

impl DataVersion for WildcardVersion {
    fn pattern(&self) -> regex::Regex {
        Regex::new(r".*").unwrap()
    }
}

pub struct IsoDateVersion {}

impl DataVersion for IsoDateVersion {
    fn pattern(&self) -> Regex {
        Regex::new(r"\d{8}").unwrap()
    }
}
