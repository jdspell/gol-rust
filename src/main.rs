use std::collections::HashMap;
// use std::str;

#[derive(Hash, Copy, Clone, Eq, Debug)]
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

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x &&
        self.y == other.y
    }
}

impl Cell {
    // creates new Cell
    fn new(neighbors: i32, live: bool) -> Cell {
        Cell { neighbors: neighbors, live: live}
    }
}

fn calc_neighbors(pt: Point, map: &HashMap<Point, Cell>) -> i32 {
    let mut num_neighbors = 0;
    let mut live_neighbors = 0;
    let mut temp_pt = pt;
    let mut temp_cell;
    // check the following 8 cases
    // x+1, y+1
    temp_pt = Point { x: pt.x+1, y: pt.y+1 };
    if map.contains_key(&temp_pt) { 
        num_neighbors += 1;
        temp_cell = map.get(&temp_pt).unwrap();
        live_neighbors += if temp_cell.live { 1 } else { 0 };
    }
    // x+1, y-1
    temp_pt = Point { x: pt.x+1, y: pt.y-1 };
    if map.contains_key(&temp_pt) { 
        num_neighbors += 1;
        temp_cell = map.get(&temp_pt).unwrap();
        live_neighbors += if temp_cell.live { 1 } else { 0 };
    }
    // x-1, y-1
    temp_pt = Point { x: pt.x-1, y: pt.y-1 };
    if map.contains_key(&temp_pt) { 
        num_neighbors += 1;
        temp_cell = map.get(&temp_pt).unwrap();
        live_neighbors += if temp_cell.live { 1 } else { 0 };
    }
    // x-1, y+1
    temp_pt = Point { x: pt.x-1, y: pt.y+1 };
    if map.contains_key(&temp_pt) { 
        num_neighbors += 1;
        temp_cell = map.get(&temp_pt).unwrap();
        live_neighbors += if temp_cell.live { 1 } else { 0 };
    }
    // x+1, y+0
    temp_pt = Point { x: pt.x+1, y: pt.y };
    if map.contains_key(&temp_pt) { 
        num_neighbors += 1;
        temp_cell = map.get(&temp_pt).unwrap();
        live_neighbors += if temp_cell.live { 1 } else { 0 };
    }
    // x-1, y+0
    temp_pt = Point { x: pt.x-1, y: pt.y };
    if map.contains_key(&temp_pt) { 
        num_neighbors += 1;
        temp_cell = map.get(&temp_pt).unwrap();
        live_neighbors += if temp_cell.live { 1 } else { 0 };
    }
    // x+0, y+1
    temp_pt = Point { x: pt.x, y: pt.y+1 };
    if map.contains_key(&temp_pt) { 
        num_neighbors += 1;
        temp_cell = map.get(&temp_pt).unwrap();
        live_neighbors += if temp_cell.live { 1 } else { 0 };
    }
    // x+0, y-1
    temp_pt = Point { x: pt.x, y: pt.y-1 };
    if map.contains_key(&temp_pt) { 
        num_neighbors += 1;
        temp_cell = map.get(&temp_pt).unwrap();
        live_neighbors += if temp_cell.live { 1 } else { 0 };
    }
    println!("Contains key: {}, Live: {}", map.contains_key(&temp_pt), live_neighbors);
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
        println!("{} neighbors ", calc_neighbors(*pt, &ptsWitNeighbors));
    }
}
