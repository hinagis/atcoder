const M: u64 = 1000_000_007;

fn main() {
    proconio::input! {
        n: usize,
        m: usize,
    }

    let c = Combination::new(m);
    let mut sign = false;
    let mut r = 0;
    for i in 0..=n {
        let k = c.comb(n, i) * c.perm(m - i, n - i) % M;
        if sign {
            if k > r {
                r += M;
            }
            r -= k;
        } else {
            r += k;
            if r > M {
                r -= M;
            }
        }
        sign ^= true;
    }

    println!("{}", r * c.perm(m, n) % M);
}

struct Combination {
    fact: Vec<u64>,
    inv_fact: Vec<u64>,
}

impl Combination {
    fn new(n: usize) -> Combination {
        let mut fact = vec![1; n + 1];
        for i in 1..=n {
            fact[i] = fact[i - 1] * i as u64 % M;
        }
        let mut inv_fact = vec![1; n + 1];
        inv_fact[n] = pow(fact[n], M - 2);
        for i in (1..=n).rev() {
            inv_fact[i - 1] = inv_fact[i] * i as u64 % M;
        }

        Combination {fact, inv_fact}
    }

    fn perm(&self, n: usize, r: usize) -> u64 {
        self.fact[n] * self.inv_fact[n - r] % M
    }
    
    fn comb(&self, n: usize, r: usize) -> u64 {
        self.perm(n, r) * self.inv_fact[r] % M
    }
}

fn pow(mut a: u64, mut n: u64) -> u64 {
    let mut r = 1;
    while n > 0 {
        if n & 1 > 0 {
            r = r * a % M;
        }
        a = a * a % M;
        n >>= 1;
    }
    r
}
