use proconio::input as I;

fn main() {
    I! {
        n: usize,
        d: i32,
        a: [i32; n]
    }

    let mut c = 0;
    let mut t = std::collections::BTreeSet::new();
    let mut r = 0;
    for l in 0..n {
        while r < n && t.range(a[r] - d + 1..a[r] + d).next().is_none() {
            t.insert(a[r]);
            r += 1;
        }
        c += r - l;
        t.remove(&a[l]);
    }

    println!("{c}");
}
