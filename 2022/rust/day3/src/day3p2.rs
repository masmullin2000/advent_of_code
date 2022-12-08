use lib::*;
use utils;

fn main() {
    let mut lines = utils::read_lines("input").unwrap();

    let mut i = 0;
    let mut elf_set = Vec::new();

    let mut total = 0;

    while let Some(Ok(line)) = lines.next() {
        elf_set.push(line);
        i += 1;

        if i % 3 == 0 {
            let badge = get_badge_item(&elf_set).unwrap();
            elf_set.clear();
            total += badge.priority() as u32;
        }
    }

    println!("sum of badges: {total}");
}
