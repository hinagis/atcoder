fn main() {
    proconio::input! {
        n: usize,
        d: i64,
        xy: [(i64, i64); n],
    }
    let d2 = d * d;

    let mut r = 0;
    for (x, y) in xy {
        if x * x + y * y <= d2 {
            r += 1;
        }
    }
    println!("{}", r);
}
