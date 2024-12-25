use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n - 1]
    }
    let mut t = vec![vec![]; n + 1];
    for &(a, b) in &ab {
        t[a].push(b);
        t[b].push(a);
    }
    for i in 1..=n {
        t[i].sort_by(|a, b| b.cmp(a));
    }

    let mut f = vec![true; n + 1];
    f[1] = false;
    dfn(&mut t, &mut f, 1);
}

fn dfn(t: &mut Vec<Vec<usize>>, f: &mut Vec<bool>, i: usize) {
    println!("{}", i);
    while let Some(e) = t[i].pop() {
        if f[e] {
            f[e] = false;
            dfn(t, f, e);
            println!("{}", i);
        }
    }
}
