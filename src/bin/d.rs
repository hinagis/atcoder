const MOD: u64 = 998244353;

fn main() {
    proconio::input! {
        n: usize,
        ab: [(u32, u32); n]
    }

    let mut f = vec![0; n + 1];
    let mut g = vec![0; n + 1];

    f[1] = 1;
    g[1] = 1;

    for i in 2..=n {
        let (a, b) = ab[i - 2];
        let (c, d) = ab[i - 1];
        if a != c {
            f[i] = f[i - 1];
        }
        if a != d {
            g[i] = f[i - 1];
        }
        if b != c {
            f[i] = (f[i] + g[i - 1]) % MOD;
        }
        if b != d {
            g[i] = (g[i] + g[i - 1]) % MOD;
        }
    }

    println!("{}", (f[n] + g[n]) % MOD);
}
