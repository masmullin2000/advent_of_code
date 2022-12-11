use utils::*;
use lib::*;

fn main() {
    let lines = read_lines("input").unwrap();
    //let lines = read_lines("test_data").unwrap();
   
    for line in lines {
        let line = line.unwrap();

        let pos = get_marker_position(&line, 4);
        println!("{pos}");
    }
}
