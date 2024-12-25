use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        wv: [(u64, u64); n],
        x: [u64; m],
        lr: [(Usize1, Usize1); q]
    }

    for i in 0..q {
        let (l, r) = lr[i];
        let mut wv: Vec<(u64, u64)> = wv.clone();
        wv.sort_by(|a, b| (b.1).cmp(&a.1));
        let mut x: Vec<u64> = x.clone();
        x.drain(l..=r);
        x.sort();
        let mut s = 0;
        for j in 0..n {
            for k in 0..x.len() {
                if x[k] >= wv[j].0 {
                    s += wv[j].1;
                    x.remove(k);
                    break;
                }
            }
        }
        println!("{}", s);
    }
}
