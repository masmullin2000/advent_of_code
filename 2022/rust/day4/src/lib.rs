use std::ops::RangeInclusive;

pub fn parse_input(input: &str) -> (RangeInclusive<u8>, RangeInclusive<u8>) {
    let vals: Vec<u8> = input
        .split(|s: char| !s.is_digit(10))
        .map(|val| val.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();

    let left = vals[0]..=vals[1];
    let right = vals[2]..=vals[3];

    (left, right)
}

pub trait Includes {
    fn includes(&self, other: &Self) -> bool;
    fn overlaps(&self, other: &Self) -> bool;
}

impl<T: std::cmp::PartialOrd> Includes for std::ops::RangeInclusive<T> {
    fn includes(&self, other: &Self) -> bool {
        self.start() <= other.start() && self.end() >= other.end()
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.start() <= other.end() && self.end() >= other.start()
    }
}
