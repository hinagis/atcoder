use itertools::Itertools;

fn main() {
    proconio::input! {
        n: usize,
        h: [usize; n]
    }
    let mut c = vec![0; n];
    let mut t = std::collections::BTreeMap::new();
    for i in 0..n {
        t.insert(h[i], i);
        for (_, &j) in t.range(h[i]..) {
            c[j] += 1;
            break;
        }
    }
    let mut r = vec![0; n];
    r[0] = c[0];
    for i in 1..n - 1 {
        r[i] = r[i - 1] - 1 + c[i];
    }
    println!("{}", r.iter().join(" "));
}
