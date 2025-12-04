use std::fs;

fn main() -> std::io::Result<()> {
    let contents = fs::read_to_string("input.txt")?;

    let matrix: Vec<&str> = contents.lines().collect();
    let mut matrix: Vec<Vec<char>> = matrix.iter().map(|r| r.to_string().chars().collect::<Vec<char>>()).collect();

    println!("count part 1: {}", part1(&matrix));
    println!("count part 2: {}", part2(&mut matrix));

    Ok(())
}

fn part1(matrix: &Vec<Vec<char>>) -> i32 {
    let mut accessible = 0;
    
    for (i, row) in matrix.iter().enumerate() {
        for j in 0..row.len() {
            if matrix[i][j] != '@' {
                continue;
            }
            let mut amount_surrounding = 0;
            let i = i as isize;
            let j = j as isize;
            let points = [[i-1, j-1], [i-1, j], [i-1, j+1],
                          [i, j-1],             [i, j+1],
                          [i+1, j-1], [i+1, j], [i+1, j+1]];
            for p in points {
                if p[0] < 0 || p[1] < 0 {
                    continue;
                }
                if let Some(val) = matrix.get(p[0] as usize).and_then(|r| r.get(p[1] as usize)).copied() {
                    if val == '@' {
                        amount_surrounding += 1;
                    }
                }
            }
            if amount_surrounding < 4 {
                accessible += 1;
                println!("accessible: ({}, {})", i, j);
            }
        }
    }
    
    accessible
}

fn part2(matrix: &mut Vec<Vec<char>>) -> i32{
    let mut accessible: Vec<(isize, isize)> = vec![];
    let mut count = 0;
    
    for (i, row) in matrix.iter().enumerate() {
        for j in 0..row.len() {
            if matrix[i][j] != '@' {
                continue;
            }
            let mut amount_surrounding = 0;
            let i = i as isize;
            let j = j as isize;
            let points = [[i-1, j-1], [i-1, j], [i-1, j+1],
                          [i, j-1],             [i, j+1],
                          [i+1, j-1], [i+1, j], [i+1, j+1]];
            for p in points {
                if p[0] < 0 || p[1] < 0 {
                    continue;
                }
                if let Some(val) = matrix.get(p[0] as usize).and_then(|r| r.get(p[1] as usize)).copied() {
                    if val == '@' {
                        amount_surrounding += 1;
                    }
                }
            }
            if amount_surrounding < 4 {
                accessible.push((i, j));
            }
        }
    }
    
    for (i, j) in &accessible {
        matrix[*i as usize][*j as usize] = '.';
    }

    count += accessible.len() as i32;

    if accessible.len() > 0 {
        count += part2(matrix);
    }

    count
}