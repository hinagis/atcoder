use proconio::{input, marker::Usize1};
use itertools::Itertools;

fn main() {
    input! {
        _n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
        cd: [(Usize1, Usize1); m],
    }
    let mut ab = ab.iter().map(|(a, b)| if a > b {(b, a)} else {(a, b)}).collect::<Vec<_>>();
    ab.sort();
    for p in (0..8).permutations(8) {
        let cd = cd.iter().map(|&(a, b)| (p[a], p[b])).collect::<Vec<_>>();
        let mut cd = cd.iter().map(|(a, b)| if a > b {(b, a)} else {(a, b)}).collect::<Vec<_>>();
        cd.sort();

        let mut f = true;
        for i in 0..m {
            if ab[i] != cd[i] {
                f = false;
                break;
            }
        }

        if f {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
