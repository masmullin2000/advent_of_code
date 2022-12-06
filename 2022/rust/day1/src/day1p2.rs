use lib;

fn main() {
    let elf_cal = lib::get_elf_calories("input"); 

    if elf_cal.len() > 3 {
        let s = &elf_cal[0..3];
        let amt = s.iter().fold(0, |acc, x| acc + x);
        println!("{}", amt);
    } else {
        println!("Error: No elf calories");
    }
}

