fn main() {
    proconio::input! {n: i64}
    let mut m = i64::max_value();
    let mut a = 0;
    let mut b = 0i64;
    while b.pow(3) < n {
        b += 1;
    }
    while a <= b {
        let a2 = a * a;
        let a3 = a2 * a;
        let b2 = b * b;
        let b3 = b2 * b;
        let x = a3 + a2 * b + a * b2 + b3;
        if x >= n {
            m = m.min(x);
            b -= 1;
        } else {
            a += 1;
        }
    }
    println!("{}", m);
}
