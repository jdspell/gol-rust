use std::collections::HashMap;
use std::str;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

struct Cell {
    neighbors: i32,
    live: bool,
}

impl Point {
    // creates new Point
    fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y}
    }
}

impl Cell {
    // creates new Cell
    fn new(neighbors: i32, live: bool) -> Cell {
        Cell { neighbors: neighbors, live: live}
    }
}

fn calc_neighbors(pt: Point, list: HashMap<Point, Cell>) -> i32 {
    let mut num_neighbors = 0;
    let mut temp_pt = pt;
    // check the following 8 cases
    // x+1, y+1
    temp_pt = Point { x: pt.x+1, y: pt.y+1 };
    if list.contains_key(temp_pt) {
        num_neighbors++;
    }
    // x+1, y-1
    temp_pt = Point { x: pt.x+1, y: pt.y-1 };
    if list.contains_key(temp_pt) {
        num_neighbors++;
    }
    // x-1, y-1
    temp_pt = Point { x: pt.x-1, y: pt.y-1 };
    if list.contains_key(temp_pt) {
        num_neighbors++;
    }
    // x-1, y+1
    temp_pt = Point { x: pt.x-1, y: pt.y+1 };
    if list.contains_key(temp_pt) {
        num_neighbors++;
    }
    // x+1, y+0
    temp_pt = Point { x: pt.x+1, y: pt.y };
    if list.contains_key(temp_pt) {
        num_neighbors++;
    }
    // x-1, y+0
    temp_pt = Point { x: pt.x-1, y: pt.y };
    if list.contains_key(temp_pt) {
        num_neighbors++;
    }
    // x+0, y+1
    temp_pt = Point { x: pt.x, y: pt.y+1 };
    if list.contains_key(temp_pt) {
        num_neighbors++;
    }
    // x+0, y-1
    temp_pt = Point { x: pt.x, y: pt.y-1 };
    if list.contains_key(temp_pt) {
        num_neighbors++;
    }
    // return the result
    return num_neighbors;
}

fn main() {
    // stores the points with neighbors
    let mut ptsWitNeighbors: HashMap<Point, Cell> = HashMap::new();

    // initial list of points to start
    ptsWitNeighbors.insert(Point::new(0, 0), Cell::new(1, true));
    ptsWitNeighbors.insert(Point::new(1, 0), Cell::new(2, true));
    ptsWitNeighbors.insert(Point::new(0, 1), Cell::new(3, true));
    ptsWitNeighbors.insert(Point::new(1, 1), Cell::new(4, true));

    // TODO: create fxn to calculate number of neighbors for each pt
    // call fxn for each pt in the loop below to set num neighbors

    // add the points to the hashmap
    for (pt, cell) in ptsWitNeighbors.iter() {
        println!("({}, {}) -> {} {} ", pt.x, pt.y, cell.neighbors, cell.live);
        println!("{} neighbors ", calc_neighbors(pt, ptsWitNeighbors));
    }
}
