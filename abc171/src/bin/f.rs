use proconio::input;

const M: u64 = 1000_000_007;

fn main() {
    input! {
        k: u64,
        s: String,
    }
    let slen = s.len() as u64;
    let e = k + slen;

    let mut r = 0;
    let mut ncr = 1;
    for n in slen..(e + 1) {
        r += ncr * (modpow(25, n - slen) * modpow(26, e - n) % M);
        r %= M;
        ncr *= n * modpow(n - slen + 1, M - 2) % M;
        ncr %= M;
    }

    println!("{}", r);
}

fn modpow(x: u64, y: u64) -> u64 {
    let mut result = 1;
    let mut a = x;
    let mut b = y;
    while b > 0 {
        if b % 2 == 1 {
            result = result * a % M;
        }
        a = a * a % M;
        b /= 2;
    }
    result
}

pub fn binom_pascal( n: u64, k: u64 ) -> u64 {
    if k == 0 || k == n {
        return 1;
    }
    let k = k as usize;

    let mut p = vec![ 1 ];

    for _ in 0..n-1 {
        let mut c = vec![ 1 ];

        for x in p.windows(2) {
            c.push( x[0] + x[1] );
        }
        c.push( 1 );

        p = c;
    }

    p[k-1] + p[k]
}
