use itertools::Itertools;
use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {n: usize}
    let mut b = vec![vec!["0".to_owned(); n]; n];
    let mut f = vec![vec![false; n]; n];
    let m = ((n + 1) / 2 - 1, (n + 1) / 2 - 1);
    b[m.0][m.1] = "T".to_owned();
    let mut d: (isize, isize) = (1, 0);
    let mut p = (0, 0);
    let mut i = 0;
    while p != m {
        i += 1;
        b[p.1][p.0] = i.to_string();
        f[p.0][p.1] = true;
        d = match d {
            (1, 0) => if p.0 >= n - 1 || f[(p.0 as isize + d.0) as usize][p.1] {(0, 1)} else {d},
            (0, 1) => if p.1 >= n - 1 || f[p.0][(p.1 as isize + d.1) as usize] {(-1, 0)} else {d},
            (-1, 0) => if p.0 == 0 || f[(p.0 as isize + d.0) as usize][p.1] {(0, -1)} else {d},
            (0, -1) => if p.1 == 0 || f[p.0][(p.1 as isize + d.1) as usize] {(1, 0)} else {d},
            _ => (1, 0)
        };
        p = ((p.0 as isize + d.0) as usize, (p.1 as isize + d.1) as usize);
    }
    println!("{}", b.iter().map(|b| b.iter().join(" ")).join("\n"));
}
