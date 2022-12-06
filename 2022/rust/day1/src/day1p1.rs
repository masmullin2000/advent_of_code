use lib;

fn main() {
    let elf_cal = lib::get_elf_calories("input"); 

    if !elf_cal.is_empty() {
        println!("{}", elf_cal[0]);
    } else {
        println!("Error: No elf calories");
    }
}

