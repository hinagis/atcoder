fn main() {
    proconio::input! {
        mut x: u64,
        k: usize,
    }

    let mut r = 1;
    for _ in 0..k {
        let t = x % (r * 10);
        x -= t;
        x += if t >= 5 * r {r * 10} else {0};
        r *= 10;
    }
    println!("{}", x);
}
