// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y), //ref 表示只借用，不移动所有权。否则下面y没有所有权
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}
