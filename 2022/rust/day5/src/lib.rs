use utils::read_lines;

#[derive(Default, Debug)]
pub struct MoveCmd {
    amt: usize,
    from_column: usize,
    to_column: usize,
}

fn split_columns(mut text: String) -> Vec<String> {
    let mut rc = Vec::new();

    while !text.is_empty() {
        let sz = if text.len() >= 4 { 4 } else { text.len() };
        let rest = text.split_off(sz);

        let inp = text
            .trim()
            .trim_matches(|c| c == '[' || c == ']')
            .to_owned();
        rc.push(inp);
        text = rest;
    }

    rc
}

fn parse_move_cmd(line: &str) -> MoveCmd {
    let data: Vec<&str> = line.split_ascii_whitespace().collect();

    MoveCmd {
        amt: data[1].parse::<usize>().unwrap(),
        from_column: data[3].parse::<usize>().unwrap() - 1,
        to_column: data[5].parse::<usize>().unwrap() - 1,
    }
}

pub fn get_input(input: &str) -> (Vec<Vec<String>>, Vec<MoveCmd>) {
    let mut lines = read_lines(input).unwrap();

    let mut set = Vec::new();

    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            break;
        } else if !line.contains("[") {
            continue;
        }
        let row = split_columns(line.to_owned());
        /*for (i, c) in row.iter().enumerate() {*/
            /*if !c.is_empty() {*/
                /*println!("{i}: {c}");*/
            /*}*/
        /*}*/

        set.push(row);
    }
    set.reverse();

    let mut len = 0;
    for s in set.iter() {
        if s.len() > len {
            len = s.len();
        }
    }

    //let mut cargo_set = Vec::new();
    let mut cargo_set = vec![Vec::new(); len];
    for row in set {
        for (i, col) in row.into_iter().enumerate() {
            if !col.is_empty() {
                cargo_set[i].push(col);
            }
        }
    }

    let mut moves = Vec::new();
    for line in lines {
        let line = line.unwrap();
        moves.push(parse_move_cmd(&line));
    }

    (cargo_set, moves)
}

pub fn print_cargo(cargo: &Vec<Vec<String>>) {
    for (i, col) in cargo.iter().enumerate() {
        let x = col.iter().fold(String::new(), |acc, i| acc + i);
        println!("{i}:{x}");
    }
}



pub fn perform_move(set: &mut Vec<Vec<String>>, mv: &MoveCmd, is_crane_9001: bool) {
    println!("{:?}", mv);
    let x = set[mv.from_column].len() - mv.amt;
    let mut crane = set[mv.from_column].split_off(x);
    if !is_crane_9001 {
        crane.reverse();
    }
    set[mv.to_column].extend(crane.into_iter());
}
