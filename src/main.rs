use std::collections::HashMap;

// struct Point {
//     x: i32,
//     y: i32,
// }
struct Point (f32, f32);

fn main() {
    // get, remove, insert
    let mut current = HashMap::<Point, i32>::new();

    // user needs to pass in a list of points to start
    // let ptOne: Point = Point { x: 0, y: 0 };
    // let ptTwo: Point = Point { x: 1, y: 0 };
    // let ptThree: Point = Point { x: 0, y: 1 };
    // let ptFour: Point = Point { x: 1, y: 1 };
    let ptOne: Point = Point(0, 0);
    let ptOne: Point = Point(1, 0);
    let ptOne: Point = Point(0, 1);
    let ptOne: Point = Point(1, 1);

    current.insert(ptOne, 1);
    current.insert(ptTwo, 2);
    current.insert(ptThree, 3);
    current.insert(ptFour, 4);

    // add the points to the hashmap
    for (pt, neighbors) in current.iter() {
        println!("({}, {}) -> {} neighbors", pt.0, pt.1, neighbors);
    }
}
