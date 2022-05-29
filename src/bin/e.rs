const M: u64 = 998244353;

fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        k: usize
    }

    let mut d = vec![1; m];
    for _ in 1..n {
        let mut c = d.clone();
        for i in 1..m {
            c[i] += c[i - 1];
        }

        for i in 0..m {
            d[i] = if i >= k {c[i - k]} else {0};
            if i + k < m {
                d[i] += c[m - 1] - c[i + if k > 0 {k - 1} else {0}];
            }
            d[i] %= M;
        }
    }

    println!("{}", d.iter().sum::<u64>() % M);
}
