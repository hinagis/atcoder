use proconio::{input as I, marker::Chars as C};

fn main() {
    let mut p: Vec<(i32, i32)> = Vec::with_capacity(100);
    for r in 1..=9 {
        I! {s: C}
        for c in 1..=9 {
            if s[c - 1] == '#' {
                p.push((r, c as i32));
            }
        }
    }
    let mut t = std::collections::HashSet::new();
    for i in 0..p.len() {
        for j in 0..p.len() {
            if j == i {
                continue;
            }

            let (a, b) = p[i];
            let (c, d) = p[j];
            let dij = (a - c).pow(2) + (b - d).pow(2);
            for k in 0..p.len() {
                if k == i || k == j {
                    continue;
                }

                let (e, f) = p[k];
                let dik = (a - e).pow(2) + (b - f).pow(2);
                let djk = (c - e).pow(2) + (d - f).pow(2);
                for l in 0..p.len() {
                    if l == i || l == j || l == k {
                        continue;
                    }

                    let (g, h) = p[l];
                    let dil = (a - g).pow(2) + (b - h).pow(2);
                    let djl = (c - g).pow(2) + (d - h).pow(2);
                    let dkl = (e - g).pow(2) + (f - h).pow(2);
                    if (dij == dik && dij == djl && dik == dkl && dil == djk)
                    || (dij == dil && dij == djk && dil == dkl && dik == djl)
                    || (dik == dil && dik == djk && dil == djl && dij == dkl)
                    {
                        let mut v = [i, j, k, l];
                        v.sort();
                        t.insert(v);
                    }
                }
            }
        }
    }
    println!("{}", t.len());
}
