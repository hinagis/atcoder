fn main() {
    proconio::input! {
        a: i64,
        b: i64,
        c: i64,
        d: i64,
    }

    let mut z = a + b;
    let mut r = c;
    let mut s = 1;

    if z > r * d && b - c * d >= 0 {
        println!("-1");
        return;
    }

    while z > r * d {
        z += b;
        r += c;
        s += 1;
    }
    println!("{}", s);
}
