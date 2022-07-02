use proconio::input as I;

fn main() {
    I! {
        n: usize,
        mut x: u128,
    }

    I! {a: u128, b: u128}
    let mut r = a + b * x;

    let mut s = a + b;
    x -= 1;
    for _ in 1..n {
        if x <= 0 {break;}

        I! {a: u128, b: u128}
        r = r.min(s + a + b * x);

        s += a + b;
        x -= 1;
    }

    println!("{}", r);
}
