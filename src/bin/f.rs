use proconio::input as I;
use std::collections::{BTreeSet, HashSet, HashMap, VecDeque};

fn main() {
    I! {
        _: usize, _: usize, n: usize,
        s: (usize, usize),
        g: (usize, usize),
    }

    let mut v: HashMap<usize, BTreeSet<usize>> = HashMap::new();
    let mut h: HashMap<usize, BTreeSet<usize>> = HashMap::new();
    for _ in 0..n {
        I! {x: usize, y: usize}
        v.entry(x).or_default().insert(y);
        h.entry(y).or_default().insert(x);
    }

    let mut d = HashSet::new();
    d.insert(s);

    let mut q = VecDeque::new();
    q.push_back((0, s));
    while let Some((c, (x, y))) = q.pop_front() {
        macro_rules! jdg {
            ($x: expr, $y: expr) => {
                let p = ($x, $y);
                if d.insert(p) {
                    if p == g {
                        println!("{}", c + 1);
                        return;
                    }
                    q.push_back((c + 1, p));
                }
            };
        }

        if let Some(v) = v.get(&x) {
            if let Some(&y) = v.range(..y).next_back() {
                jdg!(x, y + 1);
            }
            if let Some(&y) = v.range(y..).next() {
                jdg!(x, y - 1);
            }
        }

        if let Some(h) = h.get(&y) {
            if let Some(&x) = h.range(..x).next_back() {
                jdg!(x + 1, y);
            }
            if let Some(&x) = h.range(x..).next() {
                jdg!(x - 1, y);
            }
        }
    }

    println!("-1");
}
