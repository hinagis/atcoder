use itertools::Itertools;

fn main() {
    proconio::input! {
        h1: usize,
        w1: usize,
        a: [[u64; w1]; h1],
        h2: usize,
        w2: usize,
        b: [[u64; w2]; h2],
    }

    for u in (0..h1).combinations(h2) {
        for v in (0..w1).combinations(w2) {
            let mut f = true;
            for i in 0..h2 {
                for j in 0..w2 {
                    if a[u[i]][v[j]] != b[i][j] {
                        f = false;
                    }
                }
            }
            if f {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
