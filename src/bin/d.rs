fn main() {
    proconio::input! {
        n: i128,
        a: i128,
        b: i128,
    }
    let s = n * (1 + n) / 2;
    println!("{}", s - if a == 1 || b == 1 {
            s
        } else if a == b || (a < b && b % a == 0) {
            let d = n / a;
            let e = a * d;
            d * (a + e) / 2
        } else if b < a && a % b == 0 {
            let d = n / b;
            let e = b * d;
            d * (b + e) / 2
        } else {
            let c = a * b;
            let d = (n / a, n / b, n / c);
            let e = (a * d.0, b * d.1, c * d.2);
            let s = (d.0 * (a + e.0) / 2, d.1 * (b + e.1) / 2, d.2 * (c + e.2) / 2);
            s.0 + s.1 - s.2
        }
    );
}
