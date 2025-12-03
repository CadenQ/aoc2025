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
    println!("count part 1: {}", count_part1);
    println!("count part 2: {}", count_part2);

    Ok(())
}

fn part1(bank: &Vec<char>) -> i32 {
    let mut top: (usize, char) = (0, ' ');
    let mut second: char = ' ';

    for (i, c) in bank[..bank.len()-1].iter().enumerate() {
        if (*c) as u32 > top.1 as u32 {
            top = (i, *c);
        }
    }

    for c in bank[(top.0 as usize)+1..].iter() {
        if (*c) as u32 > second as u32 {
            second = *c;
        }
    }

    format!("{}{}", top.1, second).parse().unwrap()
}

fn part2(bank: &Vec<char>) -> i64 {
    let mut digits: [(usize, char); 12] = [(0, ' '); 12];

    for cur_digit in 0..digits.len() {
        // Iterate through the remaining characters in the bank to find next highest value
        // 12345678901234567
        let index = if cur_digit == 0 {
            0
        } else {
            digits[cur_digit-1].0
        };
        let upper_bound = bank.len() - 12 + index;

        for (i, c) in bank[index..upper_bound].iter().enumerate() {
            if (*c) as u32 > digits[cur_digit].1 as u32 {
                digits[cur_digit] = (i, *c);
            }
        }   
    }

    let mut final_num = String::new();
    let _ = digits.map(|f| {
        final_num.push(f.1);
    });
    println!("final num: '{}'", final_num);
    final_num.parse().unwrap()
}
