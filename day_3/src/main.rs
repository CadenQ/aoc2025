use std::fs;

fn main() -> std::io::Result<()> {
    let contents = fs::read_to_string("input.txt")?;

    let banks = contents.lines();
    let mut count_part1 = 0;
    let mut count_part2 = 0;
    banks.for_each(|r| {
        let bank: Vec<char> = r.to_string().chars().collect();
        count_part1 += part1(&bank);
        count_part2 += part2(&bank);
    });

    println!("count part 1: {count_part1}");
    println!("count part 2: {count_part2}");

    Ok(())
}

fn part1(bank: &Vec<char>) -> i32 {
    let mut top: (usize, char) = (0, ' ');
    let mut second: char = ' ';

    for (i, c) in bank[..bank.len() - 1].iter().enumerate() {
        if (*c) as u32 > top.1 as u32 {
            top = (i, *c);
        }
    }

    for c in bank[(top.0 as usize) + 1..].iter() {
        if (*c) as u32 > second as u32 {
            second = *c;
        }
    }

    format!("{}{}", top.1, second).parse().unwrap()
}

fn part2(bank: &Vec<char>) -> i64 {
    let mut digits: [(usize, char); 12] = [(0, ' '); 12];

    // Special case to handle finding first highest digit
    find_next(bank, &mut digits, &0, &0, &(bank.len() - 11));

    for cur_digit in 1..digits.len() {
        // Iterate through the remaining characters in the bank to find next highest value
        let index = digits[cur_digit - 1].0 + 1;
        let upper_bound = bank.len() - (11 - cur_digit);
        find_next(bank, &mut digits, &cur_digit, &index, &upper_bound);
    }

    let mut final_num = String::new();
    let _ = digits.map(|f| {
        final_num.push(f.1);
    });
    final_num.parse().unwrap()
}

fn find_next(
    bank: &Vec<char>,
    digits: &mut [(usize, char); 12],
    cur_digit: &usize,
    lower: &usize,
    upper: &usize,
) {
    for (i, c) in bank[*lower..*upper].iter().enumerate() {
        if (*c) as u32 > digits[*cur_digit].1 as u32 {
            digits[*cur_digit] = (i + *lower, *c);
            if *c == '9' {
                break;
            }
        }
    }
}
