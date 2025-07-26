fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        mut v: [(usize, usize); m],
    }
    v.sort_by(|a, b| b.0.cmp(&a.0));
    let mut c = vec![(n, 0)];
    for i in 0..m {
        let (a, b) = v[i];
        let d = n / a;
        let e = d * b;
        let mut p = vec![];
        for j in 0..c.len() {
            let (f, g) = c[j];
            let h = f / a;
            let k = g + h * b; 
            if k > e {
                p.push((f % a, k));
            }
        }
        if p.is_empty() {
            p.push((n % a, e));
        }
        c = p;
    }
    println!("{}", c.iter().fold(0, |a, &(_, b)| a.max(b)));
}
