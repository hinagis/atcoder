fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        lr: [(usize, usize); n]
    }
    let mut d = vec![1; m + 1];
    for (l, r) in lr {
        d[r] = d[r].max(l + 1);
    }
    for r in 1..=m {
        d[r] = d[r].max(d[r - 1]);
    }

    println!("{}", (1..=m).fold(0, |s, r| s + 1 + r - d[r]));
}
