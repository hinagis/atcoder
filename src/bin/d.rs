fn main() {
    proconio::input! {
        n: usize,
        s: f64,
        t: f64,
        p: [(i32, i32, i32, i32); n]
    }
    fn calc(c: (i32, i32), t: (i32, i32), s: f64) -> f64 {
        (((c.0 as f64) - (t.0 as f64)).powf(2.) + ((c.1 as f64) - (t.1 as f64)).powf(2.)).sqrt() / (s as f64)
    }
    let k = p.iter()
    .map(|&(a, b, c, d)| calc((a, b), (c, d), t))
    .collect::<Vec<_>>();
    let mut m = std::f64::MAX;
    let mut q = std::collections::VecDeque::new();
    q.push_front((0., (0, 0), (0..n).collect::<Vec<usize>>()));
    while let Some((a, c, r)) = q.pop_front() {
        if r.len() == 0 {
            m = m.min(a);
        }
        for i in 0..r.len() {
            let mut r = r.clone();
            let j = r.remove(i);
            let u = (p[j].0, p[j].1);
            let v = (p[j].2, p[j].3);
            q.push_back((a + k[j] + if c == u {0.} else {calc(c, u, s)}, v, r.clone()));
            q.push_back((a + k[j] + if c == v {0.} else {calc(c, v, s)}, u, r));
        }
    }
    println!("{}", m);
}
