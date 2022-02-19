fn main() {
    proconio::input! {
        x1: i64,
        y1: i64,
        x2: i64,
        y2: i64,
    }
    let q = [(1, 2), (1, -2), (2, 1), (2, -1), (-1, 2), (-1, -2), (-2, 1), (-2, -1)];

    for &a in &q {
        for &b in &q {
            if x1 + a.0 == x2 + b.0 && y1 + a.1 == y2 + b.1 {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
