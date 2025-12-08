use std::{cmp::Ordering, fs, io};

fn main() -> std::io::Result<()> {
    let contents = fs::read_to_string("input.txt")?;

    let data: Vec<&str> = contents.lines().collect();
    let data: Vec<String> = data.iter().map(|d| d.to_string()).collect();
    let mut num_of_connections = String::new();

    println!("Please enter the # of connections you'd like to make: ");
     io::stdin().read_line(&mut num_of_connections).expect("Failed to read line");

    println!("part 1 results: {}", part1(&data, num_of_connections.trim().parse().unwrap()));

    Ok(())
}

#[derive(PartialEq, Debug)]
struct Point {
    x: f64,
    y: f64,
    z: f64
}

#[derive(Debug)]
struct Connection<'a> {
    p1: &'a Point,
    p2: &'a Point,
    distance: f64
}

fn part1(data: &Vec<String>, num_of_connections: usize) -> usize {
    let junction_boxes: Vec<Point> = data.iter().map(|d| {
        let coords: Vec<&str> = d.split(',').collect();
        Point { x: coords[0].parse().unwrap(), y: coords[1].parse().unwrap(), z: coords[2].parse().unwrap() }
    }).collect();

    // let data_points: Vec<(usize, Point)> = data_points.into_iter().enumerate().collect();
    let mut connections: Vec<Connection> = vec![];

    // Establish connections
    for (i, p1) in junction_boxes.iter().enumerate() {
        for p2 in junction_boxes[i+1..].iter() {
            connections.push(Connection { p1: &p1, p2: &p2, distance: euc_dist(&p1, &p2) });
        }
    }

    connections.sort_by(|c1, c2| {
        if c1.distance < c2.distance {
            Ordering::Less
        } else if c1.distance == c2.distance {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    });

    let mut circuits: Vec<Vec<&Point>> = vec![];

    // Establish all circuits
    for jb in junction_boxes.iter() {
        circuits.push(vec![jb]);
    }

    for conn in connections[..num_of_connections].into_iter() {
        // If the two points already exist together in a circuit, ignore
        if circuits.iter().any(|circuit| circuit.contains(&conn.p1) && circuit.contains(&conn.p2)) {
            continue;
        }

        // If they don't, then we want to connect the circuits together
        let circuit_p1_index = circuits.iter().position(|c| c.contains(&conn.p1)).unwrap();
        let circuit_p2_index = circuits.iter().position(|c| c.contains(&conn.p2)).unwrap();

        let mut c_p1 = circuits[circuit_p1_index].clone();
        let mut c_p2 = circuits[circuit_p2_index].clone();
        c_p1.append(&mut c_p2);

        circuits[circuit_p1_index] = c_p1;
        circuits.remove(circuit_p2_index);
    }

    // Sort by descending order
    circuits.sort_by(|c1, c2| {
        if c1.len() < c2.len() {
            Ordering::Greater
        } else if c1.len() == c2.len() {
            Ordering::Equal
        } else {
            Ordering::Less
        }
    });

    println!("{}, {}, {}", circuits[0].len(), circuits[1].len(), circuits[2].len());
    println!("{:?}", circuits[0]);
    println!("{:?}", circuits[1]);
    println!("{:?}", circuits[2]);
    
    circuits[0].len() * circuits[1].len() * circuits[2].len()
}

fn euc_dist(p1: &Point, p2: &Point) -> f64 {
    (((p1.x - p2.x) * (p1.x - p2.x)) + ((p1.y - p2.y) * (p1.y - p2.y)) + ((p1.z - p2.z) * (p1.z - p2.z))).sqrt()
}
