fn main() {
    proconio::input! {
        a: i32,
        b: i32
    }
    let (a, b) = if a > b {(b, a)} else {(a, b)};
    println!("{}",
        match (a, b) {
            (1, 2) => 3,
            (1, 3) => 2,
            (2, 3) => 1,
            _ => -1
        }
    );
}
