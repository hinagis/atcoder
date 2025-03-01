use itertools::{iproduct, Itertools};
use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {
        n: usize,
        a: [C; n]
    }
    let mut q = std::collections::VecDeque::new();
    let mut c = vec![vec![-1; n]; n];
    for i in 0..n {
        c[i][i] = 0;
        q.push_back((i, i));
    }
    for i in 0..n {
        for j in (0..i).chain(i + 1..n) {
            if a[i][j] == '-' {continue}
            c[i][j] = 1;
            q.push_back((i, j));
        }
    }
    while let Some((i, j)) = q.pop_front() {
        for (k, l) in iproduct!(0..n, 0..n) {
            if a[k][i] != a[j][l] || a[k][i] == '-' || c[k][l] >= 0 {continue}
            c[k][l] = c[i][j] + 2;
            q.push_back((k, l));
        }
    }
    print!("{}", c.iter().map(|c| c.iter().join(" ")).join("\n"));
}
