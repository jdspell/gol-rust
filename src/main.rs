use std::collections::HashMap;
// use array_macro::array;

struct Point {
    x: i32,
    y: i32,
}


fn main() {
    // get, remove, insert
    let mut current = HashMap::<Point, i32>::new();

    // user needs to pass in a list of points to start
    let initialPts: [[Point; 4]] = [
        Point { x: 0, y: 0 },
        Point { x: 1, y: 0 },
        Point { x: 0, y: 1 },
        Point { x: 1, y: 1 },
    ];

    // add the points to the hashmap
    for pt in initialPts.iter() {
        println!("({}, {})", pt.x, pt.y);

    }
    println!();
}
