use std::fs;

fn main() -> std::io::Result<()> {
    let contents = fs::read_to_string("input.txt")?;

    let part_1 = part1(&contents);
    let part_2 = part2(&contents);
    println!("Result of part 1: {part_1}");
    println!("Result of part 2: {part_2}");
    Ok(())
}

fn part1(lines: &str) -> i16 {
    let mut dial: i16 = 50;
    let mut count: i16 = 0;
    let lines = lines.lines();
    lines.for_each(|rot| {
        let change: i16 = rot[1..].parse().unwrap();
        let change = if rot.starts_with("L") {
            change
        } else {
            change * -1
        };

        dial = (dial + change) % 100;

        if dial == 0 {
            count += 1;
        }
    });

    count
}

fn part2(lines: &str) -> i16 {
    let mut dial: i16 = 50;
    let mut count: i16 = 0;
    let lines = lines.lines();

    lines.for_each(|rot| {
        let mut change: i16 = rot[1..].parse().unwrap();

        while change != 0 {
            if rot.starts_with("L") {
                dial -= 1;
            } else {
                dial += 1;
            }

            change -= 1;

            if dial == 0 {
                count += 1;
            } else if dial == -1 {
                dial = 99;
            } else if dial == 100 {
                dial = 0;
                count += 1;
            }
        }
    });

    count
}
