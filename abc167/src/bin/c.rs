//use proconio::{input, fastout};
use proconio::input;
use itertools::*;

//#[fastout]
fn main() {
    input! {
        n: usize, m: usize, x: u32,
        mut d: [(u32, [u32; m]); n],
    }

    d.sort_by(|(c0, _), (c1, _)| c0.cmp(c1));
    let mut v = Vec::new();
    for i in 0..n {
        let c = (0..n).combinations(i + 1);
        for ci in c {
            let mut sum = 0;
            for i in &ci {
                sum += d[*i].0;
            }
            v.push((sum, ci));
        }
    }
    v.sort_by(|(c0, _), (c1, _)| c0.cmp(c1));

    for (c, i_s) in v {
        let mut sum_a = vec![0; m];
        for i in i_s {
            for j in 0..m {
                sum_a[j] += d[i].1[j];
            }
            if *sum_a.iter().min().unwrap() >= x {
                println!("{}", c);
                return;
            }
        }
    }

    println!("{}", -1);
}
