use regex::Regex;
use std::fs;

fn main() -> std::io::Result<()> {
    let contents = fs::read_to_string("input.txt")?;

    let data: Vec<&str> = contents.lines().collect();
    let data: Vec<String> = data.iter().map(|s| s.to_string()).collect();

    println!("part 1 results: {}", part1(&data));
    println!("part 2 results: {}", part2(&data));

    Ok(())
}

fn part1(rows: &Vec<String>) -> usize {
    let operators = &rows[rows.len() - 1];

    let re = Regex::new(r"[\+\*]+").unwrap();
    let operators: Vec<&str> = re.find_iter(&operators).map(|m| m.as_str()).collect();

    let mut numbers: Vec<Vec<usize>> = vec![];

    let re = Regex::new(r"[\d]+").unwrap();
    for _ in 0..operators.len() {
        numbers.push(vec![]);
    }

    for row in rows[..rows.len()].iter() {
        let matches: Vec<usize> = re
            .find_iter(row)
            .map(|m| m.as_str().parse().unwrap())
            .collect();

        for (i, m) in matches.iter().enumerate() {
            numbers[i].push(*m);
        }
    }

    cal_totals(&numbers, &operators)
}

fn part2(rows: &Vec<String>) -> usize {
    let mut col_nums: Vec<Vec<char>> = vec![];

    for _ in 0..rows[0].len() {
        col_nums.push(vec![]);
    }

    for row in rows[..rows.len() - 1].iter() {
        for (i, c) in row.chars().enumerate() {
            col_nums[i].push(c);
        }
    }

    let col_nums: Vec<String> = col_nums.iter().map(|r| r.iter().collect()).rev().collect();

    let operators = &rows[rows.len() - 1];

    let re = Regex::new(r"[\+\*]+").unwrap();
    let mut operators: Vec<&str> = re.find_iter(&operators).map(|m| m.as_str()).collect();
    operators.reverse();

    let mut numbers: Vec<Vec<usize>> = vec![];

    for _ in 0..operators.len() {
        numbers.push(vec![]);
    }

    let mut cur_index = 0;

    for entry in col_nums.iter() {
        let entry = entry.trim();
        if entry == "" {
            cur_index += 1;
            continue;
        }

        numbers[cur_index].push(entry.parse().unwrap());
    }

    cal_totals(&numbers, &operators)
}

fn cal_totals(numbers: &Vec<Vec<usize>>, operators: &Vec<&str>) -> usize {
    let mut totals: Vec<usize> = vec![];
    for (op, nums) in operators.iter().zip(numbers.iter()) {
        let total = match *op {
            "*" => nums.iter().fold(1, |acc, n| acc * *n),
            _ => nums.iter().fold(0, |acc, n| acc + *n),
        };

        totals.push(total);
    }

    totals.iter().sum()
}
