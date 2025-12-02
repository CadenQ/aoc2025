use std::fs;

fn main() -> std::io::Result<()> {
    let contents = fs::read_to_string("input.txt")?;

    let ranges: Vec<&str> = contents.split(',').collect();
    let mut ranges: Vec<String> = ranges
        .iter()
        .map(|&s| s.to_string())
        .collect();

    println!("{}", part1(&mut ranges));

    Ok(())
}

fn part1(ranges: &mut Vec<String>) -> i64 {
    let count: i64 = 0;
    let range_len = ranges.len();
    if let Some(last_string) = ranges.get_mut(range_len-1) {
        let new_val = last_string.replace('\n', "").replace('\r', "");
        *last_string = new_val;
    }
    
    println!("{:?}", ranges);   

    count
}
