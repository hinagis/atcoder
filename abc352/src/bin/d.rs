fn main() {
    proconio::input! {
        n: usize,
        k: usize,
        p: [usize; n]
    }
    let mut p = p.iter().enumerate().collect::<Vec<_>>();
    p.sort_by(|a, b| a.1.cmp(&b.1));
    let mut r = (0..k).map(|i| p[i].0).collect::<std::collections::BTreeSet<_>>();
    let mut d = r.last().unwrap() - r.first().unwrap();

    for i in k..n {
        r.remove(&p[i - k].0);
        r.insert(p[i].0);
        d = d.min(r.last().unwrap() - r.first().unwrap());
    }
    println!("{}", d);
}
