fn main() {
    proconio::input! {
        n: usize,
        d: usize,
        p: usize,
        mut f: [usize; n]
    }
    f.sort_by(|a, b| b.cmp(a));
    let mut i = 0;
    while i < n {
        if (i..n.min(i + d)).into_iter().fold(0, |s, i| s + f[i]) < p {
            break;
        }
        i += d;
    }
    println!("{}", (i / d) * p + (i..n).into_iter().fold(0, |s, i| s + f[i]));
}
