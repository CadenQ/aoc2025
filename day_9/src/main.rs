use std::fs;

fn main() -> std::io::Result<()> {
    let contents = fs::read_to_string("input.txt")?;

    let coords: Vec<&str> = contents.lines().collect();
    let coords: Vec<String> = coords.into_iter().map(|d| d.to_string()).collect();

    println!("part 1 results: {}", part1(&coords));
    Ok(())
}

fn part1(coords: &Vec<String>) -> isize {
    let red_coords: Vec<(isize, isize)> = coords.iter().map(|s| {
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

    best_area
}


// Assumptions we can make:
// 1. The green tiles will always connect the red tiles in a loop.
// 2. The inside of the loop will be filled with green tiles.
// 3. The red tiles now act as the corners for the whole perimeter of the green tiles.

fn part2(coords: &Vec<String>) {
    // Determine how to grab only rectanges with green and red tiles
    // Honestly I got no idea how to figure this out...
}