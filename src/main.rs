use std::collections::HashMap;
use std::str;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    // creates new Point
    fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y}
    }
}

fn main() {
    // stores the points with neighbors
    let mut ptsWitNeighbors = HashMap::new();

    // initial list of points to start
    ptsWitNeighbors.insert(Point::new(0, 0), 1);
    ptsWitNeighbors.insert(Point::new(1, 0), 2);
    ptsWitNeighbors.insert(Point::new(0, 1), 3);
    ptsWitNeighbors.insert(Point::new(1, 1), 4);

    // add the points to the hashmap
    for (pt, neighbors) in ptsWitNeighbors.iter() {
        println!("({}, {}) -> {} neighbors", pt.x, pt.y, neighbors);
    }
}
