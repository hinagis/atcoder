fn main() {
    proconio::input! {
        x: usize,
        y: usize,
    }
    let z = match (x, y) {
        (0, 0) => 0,
        (0, 1) => 2,
        (0, 2) => 1,
        (1, 0) => 2,
        (1, 1) => 1,
        (1, 2) => 0,
        (2, 0) => 1,
        (2, 1) => 0,
        (2, 2) => 2,
        _ => 0,
    };
    println!("{}", z);
}
