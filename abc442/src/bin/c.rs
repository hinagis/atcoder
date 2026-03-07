use proconio::{input as I, marker::Usize1 as U};

fn main() {
    I! {
        n: usize,
        m: usize
    }
    let mut c = vec![1; n];
    for _ in 0..m {
        I! {
            a: U,
            b: U
        }
        c[a] += 1;
        c[b] += 1;
    }
    println!("{}", c.iter().map(|&c| {
        let c = n - c;
        if c < 3 {0} else {c * (c - 1) * (c - 2) / 6}.to_string()
    }).collect::<Vec<_>>().join(" "));
}
