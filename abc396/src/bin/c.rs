fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        mut b: [i64; n],
        mut w: [i64; m],
    }
    b.sort_by(|a, b| b.cmp(a));
    w.sort_by(|a, b| b.cmp(a));
    let mut s = 0;
    let mut i = 0;
    while i < n && b[i] >= 0 {
        s += b[i];
        i += 1;
    }
    let mut j = 0;
    while j < m && j < i && w[j] >= 0 {
        s += w[j];
        j += 1;
    }
    while i < n && j < m {
        let t = b[i] + w[j];
        if t <= 0 {break}
        s += t;
        i += 1;
        j += 1;
    }
    println!("{}", s);
}
