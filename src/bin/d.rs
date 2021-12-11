use proconio::{input, marker::Usize1};

fn main() {
    input! {n: usize, m: usize}
    let mut d = vec![(0, [n, n]); n];
    for _ in 0..m {
        input! {a: Usize1, b: Usize1}
        if d[a].0 >= 2 || d[b].0 >= 2
        || (d[a].0 == 1 && d[b].0 == 1 && d[a].1[0] == d[b].1[0]) {
            println!("No");
            return;
        }
        let i = d[a].0;
        d[a].0 += 1;
        d[a].1[i] = b;
        let i = d[b].0;
        d[b].0 += 1;
        d[b].1[i] = a;
    }
    let mut f = vec![true; n];
    let mut i = 0;
    while i < n {
        if f[i] {
            f[i] = false;
            if d[i].0 >= 1 {
                if d[i].0 == 1 {
                    let mut p = i;
                    let mut j = d[i].1[0];
                    while d[j].0 >= 2 {
                        f[j] = false;
                        let [l, r] = d[j].1;
                        p = if p == r {l} else {r};
                        let t = p; p = j; j = t;
                    }
                    f[j] = false;
                } else {
                    let mut p = i;
                    let mut j = d[i].1[0];
                    while j != i && d[j].0 >= 2 {
                        f[j] = false;
                        let [l, r] = d[j].1;
                        p = if p == r {l} else {r};
                        let t = p; p = j; j = t;
                    }
                    f[j] = false;
                    if j == i {
                        println!("No");
                        return;
                    }
                    let mut p = i;
                    let mut j = d[i].1[1];
                    while d[j].0 >= 2 {
                        f[j] = false;
                        let [l, r] = d[j].1;
                        p = if p == r {l} else {r};
                        let t = p; p = j; j = t;
                    }
                    f[j] = false;
                }
            }
        }
        i += 1;
    }
    println!("Yes");
}
