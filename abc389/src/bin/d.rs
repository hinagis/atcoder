fn main() {
    proconio::input! {
        r: u64,
    }
    let x = |i, j| {
        (2 * i + 1) * (2 * i + 1) + (2 * j + 1) * (2 * j + 1) > 4 * r * r
    };
    let mut t = 0;
    let mut m = r - 1;
    for i in 1u64.. {
        if x(i, 1) {break}
        while x(i, m) {m -= 1}
        t += m;
    }
    println!("{}", 4 * (r - 1) + 1 + 4 * t);
}
