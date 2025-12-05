use std::{cmp::Ordering, fs};

fn main() -> std::io::Result<()> {
    let contents = fs::read_to_string("input.txt")?;

    let data: Vec<&str> = contents.lines().collect();
    let split_index = data.iter().enumerate().find(|p| (*p.1) == "").unwrap().0;

    // Retrieve all ranges as strings
    let ranges: Vec<String> = data[0..split_index].iter().map(|r| r.to_string()).collect();

    // Redefine ranges as tuples (l, u) where l is the lower bound and u is the upper bound.
    let ranges: Vec<(u64, u64)> = ranges.iter().map(|r| {
        let bounds: Vec<&str> = r.split('-').collect();
        (bounds[0].parse().unwrap(), bounds[1].parse().unwrap())
    }).collect();

    // Grab IDs for part 1
    let ids = &data[split_index+1..];
    
    println!("count part 1: {}", part1(&ranges, ids));
    println!("count part 2: {}", part2(&ranges));

    Ok(())
}

fn part1(ranges: &Vec<(u64, u64)>, ids: &[&str]) -> u64 {
    let mut amount_fresh: u64 = 0;
    
    let ids: Vec<u64> = ids.iter().map(|id| id.parse().unwrap()).collect();

    for id in ids {
        for range in ranges {
            if (range.0..=range.1).contains(&id) {
                amount_fresh += 1;
                break;
            }
        }
    }

    amount_fresh
}

fn part2(ranges: &Vec<(u64, u64)>) -> u64 {
    let mut sorted_ranges = ranges.clone();
    
    // Sorts ranges to be in ascending order. This is crucial
    // for increasing ranges in our optimized ranges later on.
    sorted_ranges.sort_by(|r1, r2| {
        if r1.0 < r2.0 {
            Ordering::Less
        } else if r1.0 == r2.0 {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    });

    // Add anchor for optimized ranges
    let mut opt_ranges: Vec<(u64, u64)> = vec![sorted_ranges[0].clone()];

    for range in sorted_ranges {
        let last_index = opt_ranges.len()-1;
        
        if (opt_ranges[last_index].0..=opt_ranges[last_index].1).contains(&range.0) {
            // 1. If range fits in the last optimized range, ignore it.
            // 2. If range's lower bound fits in the last optimized range, 
            //    but not the upper, then increase last optimized range's upper bound.
            if !(opt_ranges[last_index].0..=opt_ranges[last_index].1).contains(&range.1) {
                opt_ranges[last_index].1 = range.1;
            }
        // 3. Else, add the new range to the optimized ranges.
        } else {
            opt_ranges.push(range);
        }
    }

    // Map all ranges to how many numbers they included, then sum it up together.
    // e.g. given [1-3, 5-6], map to (3 - 1 + 1) + (6 - 5 + 1) = 3 + 2 = 5.
    opt_ranges.iter().map(|r| r.1 - r.0 + 1).sum::<u64>()
}