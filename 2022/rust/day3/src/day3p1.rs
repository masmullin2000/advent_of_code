use lib::*;
use utils;

fn main() {
    let lines = utils::read_lines("input").unwrap();

    let sum_of_priorities = lines.fold(0, |acc, line| {
        let line = line.unwrap();
        acc + if let Some(shared_item) = lib::get_shared_item(&line) {
            shared_item.priority() as u32
        } else {
            0
        }
    });

    println!("sum is {}", sum_of_priorities);
}
