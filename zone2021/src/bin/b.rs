fn main() {
    proconio::input! {
        n: usize, d: f64, h: f64,
        xy: [(f64, f64); n]
    }

    let mut r = 0f64;
    for i in 0..n {
        let (x, y) = xy[i];
        r = r.max(y - x * (h - y) / (d - x));
    }
    println!("{}", r);
}
