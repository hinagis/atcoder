fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m]
    }

    let mut t = vec![vec![]; n + 1];
    for i in 0..m {
        let (a, b) = ab[i];
        t[a].push(b);
    }

    let mut s = 0;
    for i in 1..=n {
        let mut d = std::collections::HashSet::new();
        d.insert(i);
        let mut q = vec![i];
        while let Some(j) = q.pop() {
            for &e in &t[j] {
                if ! d.contains(&e) {
                    d.insert(e);
                    q.push(e);
                }
            }
        }
        s += d.len();
    }
    println!("{}", s);
}
