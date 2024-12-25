use proconio::{input, marker::Usize1};

const R: usize = 0;

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1],
    }

    let mut g = vec![vec![]; n];
    for &(a, b) in &ab {
        g[a].push(b);
        g[b].push(a);
    }

    let mut d = vec![None; n];
    d[R] = Some(0);
    let mut q = vec![R];
    while let Some(i) = q.pop() {
        for &j in &g[i] {
            if d[j] == None {
                d[j] = Some(d[i].unwrap() + 1);
                q.push(j);
            }
        }
    }

    let mut c = vec![0; n];

    input! {q: usize}
    for _ in 0..q {
        input! {
            t: u8,
            e: Usize1,
            x: i64
        }

        let (a, b) = ab[e];
        let (b, t) = if d[a] < d[b] {(b, t)} else {(a, t ^ 3)};
        if t == 1 {
            c[0] += x;
            c[b] -= x;
        } else {
            c[b] += x;
        }
    }

    let mut q = vec![R];
    while let Some(i) = q.pop() {
        for &j in &g[i] {
            if d[i] < d[j] {
                c[j] += c[i];
                q.push(j);
            }
        }
    }

    let c = c.iter().map(|&e| e.to_string()).collect::<Vec<_>>().join("\n");
    println!("{}", c);
}
