use proconio::{input as I, marker::Usize1 as U};

fn main() {
    I! {
        n: usize,
        m: usize,
    }

    let mut f = vec![vec![false; n]; n];
    for _ in 0..m {
        I! {
            k: usize,
            x: [U; k]
        }

        for i in 0..k {
            for j in 0..k {
                f[x[i]][x[j]] = true;
                f[x[j]][x[i]] = true;
            }
        }
    }
    println!("{}", if f.iter().fold(true, |a, s| a & s.iter().fold(true, |b, e| b & e)) {"Yes"} else {"No"});
}
