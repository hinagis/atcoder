use itertools::Itertools;
use proconio::{input as I, marker::Usize1 as U};

fn main() {
    I! {
        n: usize,
        e: [(U, U)],
    }

    let mut g = vec![vec![]; n];
    let mut d = vec![0; n];
    for &(a, b) in &e {
        g[a].push(b);
        d[b] += 1;
    }

    let mut t = vec![];
    for i in 0..n {
        if d[i] == 0 {
            t.push(i);
        }
    }

    let mut r = vec![0; n];
    for i in 1..=n {
        if t.len() > i {
            println!("No");
            return;
        }

        let b = t[i - 1];
        r[b] = i;
        for &a in &g[b] {
            d[a] -= 1;
            if d[a] == 0 {
                t.push(a);
            }
        }
    }
    println!("Yes\n{}", r.iter().join(" "));
}
