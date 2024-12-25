use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {
        n: usize,
        a: [C; n]
    }
    let mut r = Vec::with_capacity(n * n * 8);
    let d = [(0, 1), (0, -1), (1, 0), (-1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1)];
    for i in 0..n {
        for j in 0..n {
            for &d in &d {
                let mut t = String::with_capacity(n);
                let (mut u, mut v) = (i, j); 
                for _ in 0..n {
                    t.push(a[u][v]);
                    u = (((u + n) as isize + d.0) as usize) % n;
                    v = (((v + n) as isize + d.1) as usize) % n;
                }
                r.push(t);
            }
        }
    }

    println!("{}", r.iter().fold("0", |m, t| m.max(t)));
}
