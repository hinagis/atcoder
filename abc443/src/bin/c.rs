use proconio::input as I;

fn main() {
    I! {
        n: usize,
        t: u32,
        a: [u32; n]
    }
    if n == 0 {
        println!("{}", t);
        return;
    }

    let mut s = a[0];
    let mut p = a[0];
    for i in 1..n {
        let d = a[i] - p;
        if d <= 100 {continue}
        s += d - 100;
        p = a[i];
    }
    let d = t - p;
    if d > 100 {
        s += d - 100;
    }

    println!("{}", s);
}
