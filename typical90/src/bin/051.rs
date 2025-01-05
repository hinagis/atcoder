fn main() {
    proconio::input! {
        n: usize,
        k: usize,
        p: u64,
        a: [u64; n]
    }
    let u = calc(k, p, &a[..n / 2], false);
    let v = calc(k, p, &a[n / 2..], true);
    let mut m = 0;
    for i in 0..u.len() {
        for j in 0..u[i].len() {
            m += match v[k - i].binary_search(&(p - u[i][j] + 1)) {Ok(j) => j, Err(j) => j};
        }
    }

    println!("{}", m);
}

fn calc(k: usize, p: u64, a: &[u64], s: bool) -> Vec<Vec<u64>> {
    let mut r = vec![vec![]; k + 1];
    r[0].push(0);
    for &a in a.iter() {
        for i in (0..k).rev() {
            for j in 0..r[i].len() {
                let a = r[i][j] + a;
                if a <= p {
                    r[i + 1].push(a);
                }
            }
        }
    }
    if s {
        for i in 0..=k {
            r[i].sort();
        }
    }
    r
}