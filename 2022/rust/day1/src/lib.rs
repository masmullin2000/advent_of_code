pub fn get_elf_calories(input: &str) -> Vec<i64> {
    let lines = utils::read_lines(input).unwrap();

    let mut elf_cal = Vec::new();
    let mut i = 0;

    elf_cal.push(0);

    lines
        .filter_map(|line| match line {
            Ok(line) => Some(line),
            _ => None,
        })
        .for_each(|line| {
            if line.is_empty() {
                elf_cal.push(0);
                i += 1;
            } else {
                elf_cal[i] += line.parse::<i64>().unwrap();
            }
        });

    elf_cal.sort_unstable_by(|a,b| b.cmp(a));
    elf_cal
}
