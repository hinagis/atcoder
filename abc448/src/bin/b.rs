use proconio::{input as I, marker::Usize1 as U};

fn main() {
    I! {
        n: usize,
        m: usize,
        mut c: [u32; m]
    }
    let mut s = 0;
    for _ in 0..n {
        I! {
            a: U,
            b: u32
        }
        s += c[a].min(b);
        c[a] -= c[a].min(b);
    }
    println!("{}", s);
}
