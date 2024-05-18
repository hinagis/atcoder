use itertools::Itertools;

fn main() {
    proconio::input! {
        n: usize,
        ac: [(u64, u64); n]
    }
    let mut ac = ac.iter().enumerate().map(|(i, &(a, c))| (a, c, i + 1)).collect::<Vec<_>>();
    ac.sort();
    let mut v = vec![0; n];
    v[n - 1] = ac[n - 1].1;
    for i in (0..n - 1).rev() {
        v[i] = v[i + 1].min(ac[i].1);
    }
    let mut r = vec![ac[n - 1].2];
    for i in 0..n - 1 {
        if ac[i].1 <= v[i + 1] {
            r.push(ac[i].2);
        }
    }
    r.sort();
    println!("{}\n{}", r.len(), r.iter().join(" "));
}
