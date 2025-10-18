fn main() {
    proconio::input! {
        s: u32,
        a: u32,
        b: u32,
        mut x: u32,
    }
    let mut c = 0;
    while x >= a {
        c += s * a;
        x -= a;
        if x >= b {
            x -= b;
        } else {
            x = 0;
        }
    }
    c += s * x;
    println!("{}", c);
}
