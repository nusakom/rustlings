#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let optional_point = Some(Point { x: 100, y: 200 });

    // Using `ref p` to avoid moving the Point out of the Some variant
    match optional_point {
        Some(ref p) => println!("Coordinates are {},{}", p.x, p.y),
        // The underscore `_` is not necessary here since `optional_point` is a `Some` variant
        // But it's kept here to match the original code structure
        _ => panic!("No match!"),
    }

    // This line will print the debug representation of `optional_point`
    println!("{optional_point:?}");
}
