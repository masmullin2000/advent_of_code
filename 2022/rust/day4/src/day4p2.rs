use std::ops::RangeInclusive;

use lib::*;
use utils::*;

fn main() {
    let input = read_lines("input").unwrap();

    let acc_help = |x: RangeInclusive<u8>, y: RangeInclusive<u8>| {
        if x.overlaps(&y) {
            1
        } else {
            0
        }
    };

    let amt = input
        .map(|line| {
            parse_input(&line.unwrap())
        })
        .fold(0, |acc, (left, right)| acc + acc_help(left, right));

    println!("Amt is {amt}");
}
