use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m]
    }

    let mut c = vec![3u64; n];
    for i in 0..n {
        for j in 0..m {
            let (a, b) = ab[j];
            let (a, b) = if b == i {(b, a)} else {(a, b)};
            if a == i {
                if b > i {
                    c[b] = c[b].saturating_sub(1);
                }
            }
        }
    }
    println!("{}", c.iter().fold(1, |r, c| r * c))
}
