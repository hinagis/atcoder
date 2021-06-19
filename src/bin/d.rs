fn main() {
    proconio::input! {
        n: usize,
        a: [usize; n]
    }
    let mut t = std::collections::HashMap::new();
    for i in 0..n / 2 {
        if a[i] != a[n - i - 1] {
            let (a0, a1) = (a[i].min(a[n - i - 1]), a[i].max(a[n - i - 1]));
            t.entry(a0).or_insert(std::collections::HashSet::new()).insert(a1);
        }
    }
    let mut c = 0;
    while ! t.is_empty() {
        c += 1;
        let a = *t.keys().last().unwrap();
        let mut e = t.remove(&a).unwrap();
        let b = *e.iter().last().unwrap();
        e.remove(&b);
        for v in e.iter() {
            let (a0, a1) = (b.min(*v), b.max(*v));
            t.entry(a0).or_insert(std::collections::HashSet::new()).insert(a1);
        }
    }

    println!("{}", c);
}
