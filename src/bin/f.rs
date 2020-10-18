const M: u64 = 1000_000_007;

fn main() {
    proconio::input! {
        n: usize,
        lr: [(u64, u64); n],
    }
    let mut ml = 0;
    let mut pq = vec![];
    for &(l, r) in &lr {
        ml = ml.max(l);
        if let Err(i) = pq.binary_search(&l) {pq.insert(i, l)}
        if let Err(i) = pq.binary_search(&r) {pq.insert(i, r)}
    }

    let mut ans = 0;
    let mut dp = vec![1];
    let invs = make_inv(n + 1);
    for pq in pq.windows(2).rev() {
        let (p, q) = (pq[0], pq[1]);
        if q == ml {
            break
        }
        for &(l, r) in &lr {
            if r == q {
                let d = inv(r - l);
                let mut next = vec![0; dp.len() + 1];
                for (i, &v) in dp.iter().enumerate() {
                    next[i] = (next[i] + M - ((v * d % M) * l % M)) % M;
                    next[i + 1] = (next[i + 1] + (v * d % M)) % M;
                }
                dp = next;
            }
        }
        let func = |x: u64| -> u64 {
            let mut res = 0;
            for (i, &a) in dp.iter().enumerate().skip(1).rev() {
                res = ((res * x % M) + ((a * i as u64 % M) * invs[i + 1] % M)) % M;
            }
            (res * x % M) * x % M
        };
        ans = ((ans + func(q) % M) + M - func(p)) % M;
    }
    for i in 0..n {
        ans = ans * (i as u64 + 2) % M;
    }
    for p in lr.iter() {
        ans = ans * (p.1 - p.0) % M;
    }
    println!("{}", ans);
}

fn make_inv(n: usize) -> Vec<u64> {
    let mut f = vec![1; n + 1];
    for i in 2..=n {
        f[i] = f[i - 1] * i as u64 % M;
    }
    let mut ifact = inv(f[n]);
    let mut invs = vec![1; n + 1];
    invs[n] = ifact * f[n - 1] % M;
    for i in (1..n).rev() {
        ifact = ifact * (i as u64 + 1) % M;
        invs[i] = ifact * f[i - 1] % M;
    }
    invs
}

fn inv(x: u64) -> u64 {
    modpow(x, M - 2)
}

pub fn modpow(mut x: u64, mut n: u64) -> u64 {
    let mut r = 1;
    while n > 0 {
        if n & 1 == 1 {
            r = r * x % M;
        }
        x = x * x % M;
        n >>= 1;
    }
    r
}
