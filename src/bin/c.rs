use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
        k: usize,
        cd: [(Usize1, Usize1); k],
    }

    let mut max = 0;
    for t in 0..(1 << k) {
        let mut dish = vec![false; n];
        for i in 0..k {
            let (c, d) = cd[i];
            dish[if t & (1 << i) == 0 {c} else {d}] = true;
        }
        let mut c = 0;
        for i in 0..m {
            let (a, b) = ab[i];
            if dish[a] && dish [b] {c += 1}
        }
        max = max.max(c)
    }

    println!("{}", max);
}
