use std::fs;

fn main() -> std::io::Result<()> {
    let contents = fs::read_to_string("input.txt")?;

    let banks = contents.lines();
    let mut count_part1 = 0;
    banks.for_each(|r| {
        let bank: Vec<char> = r.to_string().chars().collect();
        count_part1 += part1(&bank);
    });
    println!("count part 1: {}", count_part1);

    Ok(())
}

fn part1(bank: &Vec<char>) -> i32 {
    let mut top: (i32, char) = (-1, ' ');
    let mut second: char = ' ';

    for (i, c) in bank[..bank.len()-1].iter().enumerate() {
        if (*c) as u32 > top.1 as u32 {
            top = (i as i32, *c);
        }
    }

    for c in bank[(top.0 as usize)+1..].iter() {
        if (*c) as u32 > second as u32 {
            second = *c;
        }
    }

    format!("{}{}", top.1, second).parse().unwrap()
}
