use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
        b: [Usize1; n],
        c: [Usize1; n],
    }

    let mut v = vec![0u64; n];
    for i in 0..n {
        v[a[i]] += 1;
    }

    let mut s = 0;
    for j in 0..n {
        s += v[b[c[j]]];
    }

    println!("{}", s);
}
