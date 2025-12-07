use std::fs;

fn main() -> std::io::Result<()> {
    let contents = fs::read_to_string("input.txt")?;

    let data: Vec<&str> = contents.lines().collect();
    let data: Vec<String> = data.iter().map(|s| s.to_string()).collect();

    println!("part 1 result: {}", part1(&data));
    println!("part 2 result: {}", part2(&data));
    Ok(())
}

fn part1(layers: &Vec<String>) -> usize {
    let beam_index = layers[0].find('S').unwrap();

    let mut beam_register: Vec<char> = layers[1].chars().collect();

    // Init beam activity
    beam_register[beam_index] = '|';
    let mut amount_split: usize = 0;

    for layer in layers[2..].iter().step_by(2) {
        let mut beam_split_indices: Vec<usize> = vec![];
        let layer: Vec<char> = layer.chars().collect();

        for (i, col) in beam_register.iter().enumerate() {
            if *col != '|' {
                continue;
            }

            if layer[i] == '^' {
                beam_split_indices.push(i);
            }
        }

        for spl in beam_split_indices {
            beam_register[spl] = '.';
            beam_register[spl - 1] = '|';
            beam_register[spl + 1] = '|';
            amount_split += 1;
        }
    }

    amount_split
}

// Solution is very close to part1!
// Main difference from part1: part1 just wanted to keep
// track of how many times you split. part2 wants you to
// keep track of how many ways there are to get to the
// bottom. So if there's 1 way to get to a splitter, then
// naturally the routes that follow the splitter would be
// 1 each. If there's 2 ways to get to a splitter, then
// there's 2 ways to get to each side of the splitter
// (4 total). E.g.
//
//               S
//               |
//             | ^ |
//            |^| |^|
//
// In the regular graph, that last line would've looked like
// this: |^|^| instead of |^||^|. That's because the middle
// beam would've overlapped, but in terms of routes there
// are two routes in the middle. That is the logic the
// calculations in my part2 solution follow.
//
// Here's a bigger example, first drawn like the aoc diagrams:
//
//                 S
//                 |
//                |^|
//                | |
//               |^|^|
//               | | |
//              |^|^|^|
//              | | | |
//              ||^|| |
//              || || |
//
// If we decide to display all routes at the same time,
// using the same visualisation of seeing all possible routes
// at the same time, pay attention to all the lines that arrive
// at the bottom:
//
//                   S
//                   |
//               |   ^   |
//               |       |
//            |  ^  | |  ^  |
//            |     | |     |
//          | ^ | || ^ || | ^ |
//          |   | ||   || |   |
//          |||| ^ ||| || |   |
//          ||||   ||| || |   |
//
// If you count the total number of lines that made it to the
// bottom when visualizing all outcomes at the same time, you
// will notice that the number of lines at the bottom is 11.
// This is the exact number of ways that it is possible to reach
// the bottom with this splitter setup. Also note that for however
// many # of beams that hit a splitter, that number of beams gets
// split to both sides. So if a splitter is hit with 2 beams, then
// 2 beams result out of each side. e.g.
//
//     | |
//   || ^ ||
//
// Hopefully this explains my logic, or maybe I just wasted 5 minutes
// of your time. <3
fn part2(layers: &Vec<String>) -> usize {
    let beam_index = layers[0].find('S').unwrap();

    let mut beam_register: Vec<usize> = vec![0; layers[0].len()];

    // Init beam activity
    beam_register[beam_index] = 1;

    for layer in layers[2..].iter().step_by(2) {
        let mut beam_split_indices: Vec<usize> = vec![];
        let layer: Vec<char> = layer.chars().collect();

        for (i, col) in beam_register.iter().enumerate() {
            if *col == 0 {
                continue;
            }

            if layer[i] == '^' {
                beam_split_indices.push(i);
            }
        }

        for spl in beam_split_indices {
            let cur_beams = beam_register[spl];
            beam_register[spl] = 0;
            beam_register[spl - 1] += cur_beams;
            beam_register[spl + 1] += cur_beams;
        }
    }

    beam_register.iter().sum()
}
