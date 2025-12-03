use std::fs;

fn main() -> std::io::Result<()> {
    let contents = fs::read_to_string("input.txt")?;

    let ranges = contents.lines();
    println!("Hello, world!");

    Ok(())
}
