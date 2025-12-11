use std::fs;

fn main() -> std::io::Result<()> {
    let contents = fs::read_to_string("input.txt")?;

    let wires: Vec<&str> = contents.lines().collect();
    let wires: Vec<String> = wires.into_iter().map(|d| d.to_string()).collect();




    Ok(())
}

fn part1(wires: &Vec<String>) -> usize {


    0
}
