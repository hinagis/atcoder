use proconio::input;

fn main() {
    input! {n: usize}

    let mut m = 0;
    let mut u = 0;
    let mut c = 0;
    for _ in 0..n {
        input! {x: i64}
        let x = x.abs();
        m += x;
        u += x * x;
        c = c.max(x);
    }
    println!("{}", m);
    println!("{}", (u as f64).sqrt());
    println!("{}", c);
}
