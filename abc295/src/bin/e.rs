use std::ops;

const MOD: i64 = 998244353;

fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        k: usize,
        a: [usize; n],
    }

    let mut z = 0;
    let mut s = vec![0; m + 1];
    for &t in &a {
        if t == 0 {
            z += 1;
        } else {
            s[t] += 1;
        }
    }

    for i in 1..=m {
        s[i] += s[i - 1];
    }

    let mut f = vec![ModInt::new(1); n + 1];
    for i in 1..=n {
        f[i] = f[i - 1] * ModInt::new(i as i64);
    }

    let comb = |n: usize, r: usize| f[n] / (f[r] * f[n - r]);

    let mut p = vec![ModInt::new(0); m + 1];
    for i in 1..=m {
        if k >= s[i] {
            for j in k - s[i]..=z {
                p[i] += comb(z, j)
                    * ModInt::new(i as i64).pow(j as i64)
                    * ModInt::new((m - i) as i64).pow((z - j) as i64)
                    / ModInt::new(m as i64).pow(z as i64);
            }
        } else {
            p[i] = ModInt::new(1);
        }
    }

    println!("{}", (1..=m).fold(ModInt::new(0), |s, i| s + (p[i] - p[i - 1]) * ModInt::new(i as i64)).val);
}

#[derive(Clone, Copy)]
struct ModInt {
    val: i64,
}

impl ModInt {
    fn new(v: i64) -> Self {
        let mut r = v % MOD;
        if r < 0 {
            r += MOD;
        }
        Self { val: r }
    }

    fn pow(&self, mut n: i64) -> Self {
        let mut v = self.val;
        let mut res = 1;
        while n != 0 {
            if n % 2 == 1 {
                res = res * v % MOD;
            }
            v = v * v % MOD;
            n /= 2;
        }
        Self { val: res }
    }
    fn inv(&self) -> Self {
        self.pow(MOD - 2)
    }
}

impl ops::Add for ModInt {
    type Output = ModInt;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            val: if self.val + rhs.val >= MOD {
                self.val + rhs.val - MOD
            } else {
                self.val + rhs.val
            },
        }
    }
}
impl ops::AddAssign for ModInt {
    fn add_assign(&mut self, rhs: Self) {
        *self = ops::Add::add(*self, rhs);
    }
}
impl ops::Sub for ModInt {
    type Output = ModInt;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            val: if self.val - rhs.val < 0 {
                self.val - rhs.val + MOD
            } else {
                self.val - rhs.val
            }
        }
    }
}
impl ops::Mul for ModInt {
    type Output = ModInt;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            val: self.val * rhs.val % MOD,
        }
    }
}
impl ops::Div for ModInt {
    type Output = ModInt;
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inv()
    }
}
