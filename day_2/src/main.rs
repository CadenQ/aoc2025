use std::fs;

fn main() -> std::io::Result<()> {
    let contents = fs::read_to_string("input.txt")?;

    let ranges: Vec<&str> = contents.split(',').collect();
    let mut ranges: Vec<String> = ranges.iter().map(|&s| s.to_string()).collect();
    let range_len = ranges.len();

    if let Some(last_string) = ranges.get_mut(range_len - 1) {
        let new_val = last_string.replace('\n', "").replace('\r', "");
        *last_string = new_val;
    }

    let mut count_part1: i64 = 0;
    let mut count_part2: i64 = 0;
    ranges.iter().for_each(|r| {
        // Define the lower and upper bounds
        let bounds: Vec<&str> = r.split('-').collect();
        let lower: i64 = bounds[0].parse().unwrap();
        let upper: i64 = bounds[1].parse().unwrap();

        // Iterate through every number from lower to upper bounds
        part1(&lower, &upper, &mut count_part1);
        part2(&lower, &upper, &mut count_part2);
    });
    println!("part1 result: {}", count_part1);
    println!("part2 result: {}", count_part2);

    Ok(())
}

// Rules:
// 1. The exact same sequence of digits must repeat exactly twice in a given number
fn part1(lower: &i64, upper: &i64, count: &mut i64) {
    for i in *lower..=*upper {
        // Convert num to a string that I can split in two
        let num_string = i.to_string();
        if num_string.len() % 2 != 0 {
            continue;
        }

        // Splits the number in half
        let half = num_string.len() / 2;
        let left = &num_string[0..half];
        let right = &num_string[half..];

        // If the left is the same as the right, add the number
        if left == right {
            *count += i;
        }
    }
}

// Rules:
// 1. The entire number has to be made up completely of some repeating sequence of digits
// 2. This sequence of repeating digits has to repeat at least twice
fn part2(lower: &i64, upper: &i64, count: &mut i64) {
    for i in *lower..=*upper {
        count_repeats(count, &i);
    }
}

fn count_repeats(count: &mut i64, i: &i64) {
    // Not worth computing if it's a single digit
    if *i < 10i64 {
        return;
    }

    let num_string = (*i).to_string();
    let num_len = num_string.len();

    // The repeat can only be at maximum half the size of the number
    let half = num_len / 2;
    let mut buf_len: usize = 1;
    let mut successful = true;

    // The buffer can be no larger than half the number's length
    while buf_len <= half {
        // Grabs the first repetition
        let slice = &num_string[0..buf_len];
        successful = true;

        // Verifies if all subsequent slices of size buf_len match the first slice
        for j in (buf_len..num_len).step_by(buf_len) {
            // if the buffer goes out of bounds the slice did not repeat correctly
            if j + buf_len > num_len {
                successful = false;
                break;
            }

            let next_slice = &num_string[j..j + buf_len];

            // If the next_slice doesn't equal the original slice, the slice didn't repeat correctly
            if slice != next_slice {
                successful = false;
                break;
            }
        }

        // If all slices repeated correctly, this is an invalid ID
        if successful {
            break;
        }
        buf_len += 1;
    }

    if successful {
        *count += i;
    }
}
