use lib::*;
use utils::read_lines;

pub fn get_score_line(line: &str) -> u8 {
    let mut parser = line.chars();

    let theirs = RPSChoice::new(parser.next().unwrap());
    parser.next();
    let mine = DesiredOutcome::new(parser.next().unwrap());

    get_score(theirs, mine.get_rps_choice(theirs))
}

fn main() {
    let lines = read_lines("input").unwrap();

    let total = lines.fold(0, |acc, line| acc + get_score_line(&line.unwrap()) as u64);
    println!("total: {total}");
}
