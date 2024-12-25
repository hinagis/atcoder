fn main() {
    proconio::input! {
        n: usize,
        xy: [(i64, i64); n],
    }

    let mut max0 = i64::min_value();
    let mut min0 = i64::max_value();
    let mut max1 = i64::min_value();
    let mut min1 = i64::max_value();
    for i in 0..n {
        let (x, y) = xy[i];
        max0 = max0.max(x - y);
        min0 = min0.min(x - y);
        max1 = max1.max(x + y);
        min1 = min1.min(x + y);
    }

    println!("{}", (max0 - min0).max(max1 - min1));
}
