use lib;

fn main() {
    let (mut set, moves) = lib::get_input("input");
    //let (mut set, moves) = lib::get_input("test_input");

    for mv in moves {
        //println!("{i}");
        lib::print_cargo(&set);
        lib::perform_move(&mut set, &mv, true);
    }

    let rc = set.iter().fold(String::new(), |acc, rc| {
        acc + &rc[rc.len()-1]
    });

    println!("{rc}");
}
