fn main() {
    proconio::input! {
        mut x: u64,
        y: u64,
        a: u64,
        b: u64,
    }

    let mut e = 0;
    let a_lim = y.min(x + b);
    if x < a_lim / a {
        while x * a <= a_lim {
            e += 1;
            x *= a;
        }
    }
    e += (y - x - 1) / b;
    println!("{}", e);
}
