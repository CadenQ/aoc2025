use std::fs;

fn main() -> std::io::Result<()> {
    let contents = fs::read_to_string("input.txt")?;

    let grid: Vec<&str> = contents.lines().collect();
    let grid: Vec<String> = grid.iter().map(|d| d.to_string()).collect();


    println!("part 1 results: {}", part1(&grid));
    Ok(())
}

fn part1(grid: &Vec<String>) -> usize {
    let red_coords: Vec<(isize, isize)> = grid.iter().map(|s| {
        let coords: Vec<&str> = s.split(',').collect();
        (coords[0].parse().unwrap(), coords[1].parse().unwrap())
    }).collect();

    let mut best_area: isize = 0;
    for (i, c1) in red_coords.iter().enumerate() {
        for c2 in red_coords[i+1..].iter() {
            let length = (c1.0 - c2.0).abs() + 1;
            let width = (c1.1 - c2.1).abs() + 1;
            if length * width > best_area {
                best_area = length * width;
            }
        }
    }

    best_area as usize
}
