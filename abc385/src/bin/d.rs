use std::collections::{HashSet as S, HashMap as M, BTreeSet as B};

fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        mut x: i64,
        mut y: i64,
        xy: [(i64, i64); n],
        dc: [(char, i64); m],
    }
    let mut h = S::new();
    let mut u = M::new();
    let mut v = M::new();
    for (x, y) in xy {
        u.entry(x).or_insert(B::new()).insert(y);
        v.entry(y).or_insert(B::new()).insert(x);
    }
    for (d, c) in dc {
        match d {
            'L' => {
                let n = x - c;
                if let Some(v) = v.get(&y) {
                    let v = v.range(n..x).collect::<Vec<_>>();
                    for &x in v {
                        h.insert((x, y));
                    }
                }
                x = n;
            }
            'R' => {
                let n = x + c;
                if let Some(v) = v.get(&y) {
                    let v = v.range(x..=n).collect::<Vec<_>>();
                    for &x in v {
                        h.insert((x, y));
                    }
                }
                x = n;
            }
            'D' => {
                let n = y - c;
                if let Some(u) = u.get(&x) {
                    let u = u.range(n..=y).collect::<Vec<_>>();
                    for &y in u {
                        h.insert((x, y));
                    }
                }
                y = n;
            }
            'U' => {
                let n = y + c;
                if let Some(u) = u.get(&x) {
                    let u = u.range(y..=n).collect::<Vec<_>>();
                    for &y in u {
                        h.insert((x, y));
                    }
                }
                y = n;
            }
            _ => ()
        }
    }

    println!("{x} {y} {}", h.len());
}
