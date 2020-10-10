fn main() {
    proconio::input! {
        n: usize,
        p: [usize; n],
    }
    let mut t = std::collections::BTreeSet::new();
    for i in 0..=n {
        t.insert(i);
    }
    for i in 0..n {
        t.remove(&p[i]);
        println!("{}", t.iter().nth(0).unwrap());
    }
}
