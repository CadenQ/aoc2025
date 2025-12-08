use std::{cmp::Ordering, fs, io};

fn main() -> std::io::Result<()> {
    let contents = fs::read_to_string("input.txt")?;

    let data: Vec<&str> = contents.lines().collect();
    let data: Vec<String> = data.iter().map(|d| d.to_string()).collect();
    let mut num_of_connections = String::new();

    println!("Please enter the # of connections you'd like to make: ");
    io::stdin().read_line(&mut num_of_connections).expect("Failed to read line");

    println!("part 1 results: {}", part1(&data, num_of_connections.trim().parse().unwrap()));
    println!("part 2 results: {}", part2(&data));

    Ok(())
}

#[derive(PartialEq, Debug)]
struct Point {
    x: f64,
    y: f64,
    z: f64
}

#[derive(Clone, Debug)]
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

    let mut connections: Vec<Connection> = vec![];

    // Establish connections
    for (i, p1) in junction_boxes.iter().enumerate() {
        for p2 in junction_boxes[i+1..].iter() {
            connections.push(Connection { p1: &p1, p2: &p2, distance: euc_dist(&p1, &p2) });
        }
    }

    // Sort connections so that the shortest connections are at the start of the list
    connections.sort_by(|c1, c2| {
        match c1.distance {
            d if d < c2.distance => Ordering::Less,
            d if d == c2.distance => Ordering::Equal,
            _ => Ordering::Greater
        }
    });

    // Establish all circuits
    let mut circuits: Vec<Vec<&Point>> = vec![];
    for jb in junction_boxes.iter() {
        circuits.push(vec![jb]);
    }

    // Connect each circuit for the shortest num_of_connections connections
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
        match c1.len() {
            l if l < c2.len() => Ordering::Greater,
            l if l == c2.len() => Ordering::Equal,
            _ => Ordering::Less
        }
    });

    circuits[0].len() * circuits[1].len() * circuits[2].len()
}

fn euc_dist(p1: &Point, p2: &Point) -> f64 {
    (((p1.x - p2.x) * (p1.x - p2.x)) + ((p1.y - p2.y) * (p1.y - p2.y)) + ((p1.z - p2.z) * (p1.z - p2.z))).sqrt()
}

fn part2(data: &Vec<String>) -> f64 {
    let junction_boxes: Vec<Point> = data.iter().map(|d| {
        let coords: Vec<&str> = d.split(',').collect();
        Point { x: coords[0].parse().unwrap(), y: coords[1].parse().unwrap(), z: coords[2].parse().unwrap() }
    }).collect();

    let mut connections: Vec<Connection> = vec![];

    // Establish connections
    for (i, p1) in junction_boxes.iter().enumerate() {
        for p2 in junction_boxes[i+1..].iter() {
            connections.push(Connection { p1: &p1, p2: &p2, distance: euc_dist(&p1, &p2) });
        }
    }

    // Sort connections so that the shortest connections are at the start of the list
    connections.sort_by(|c1, c2| {
        match c1.distance {
            d if d < c2.distance => Ordering::Less,
            d if d == c2.distance => Ordering::Equal,
            _ => Ordering::Greater
        }
    });

    // Establish all circuits
    let mut circuits: Vec<Vec<&Point>> = vec![];
    for jb in junction_boxes.iter() {
        circuits.push(vec![jb]);
    }

    let mut last_connection: Connection = Connection { p1: &Point { x: 0f64, y: 0f64, z: 0f64 }, p2: &Point { x: 0f64, y: 0f64, z: 0f64 }, distance: 0f64 };
    // Connect each circuit for the shortest num_of_connections connections
    for (i, conn) in connections.iter().enumerate() {
        // If all junction boxes are on one circuit, stop connecting
        if circuits.len() == 1 {
            last_connection = connections[i-1].clone();
            break;
        }

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

    // Grab last connection made and multiply the x coordinates
    last_connection.p1.x * last_connection.p2.x 
}