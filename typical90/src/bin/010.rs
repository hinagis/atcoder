fn main() {
    proconio::input! {
        n: usize,
        cp: [(usize, u32); n],
        q: usize,
        lr: [(usize, usize); q]
    }
    let mut s = vec![[0, 0]; n + 1];
    for i in 0..n {
        let (c, p) = cp[i];
        s[i + 1][c - 1] += p;
        for c in 0..=1 {
            s[i + 1][c] += s[i][c];
        }
    }

    for j in 0..q {
        let (l, r) = lr[j];
        println!("{} {}", s[r][0] - s[l - 1][0], s[r][1] - s[l - 1][1]);
    }
}
