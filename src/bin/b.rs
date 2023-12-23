fn main() {
    proconio::input! {
        a: i64,
        m: i64,
        l: i64,
        r: i64
    }
    println!("{}", if a < l {
        (r - a).abs() / m - (l - 1 - a).abs() / m
    } else if a > r {
        (a - l).abs() / m - (a - (r + 1)).abs() / m
    } else {
        (r - a).abs() / m + (a - l).abs() / m + 1
    });
}
