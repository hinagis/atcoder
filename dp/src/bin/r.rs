const M: u64 = 1000_000_007;

fn main() {
    proconio::input! {
        n: usize,
        k: u64,
        a: [[u64; n]; n],
    }

    let a = pow(a, k);

    let mut r = 0;
    for i in 0..n {
        for j in 0..n {
            r += a[i][j];
            r %= M;
        }
    }

    println!("{}", r);
}

fn pow(mut a: Vec<Vec<u64>>, mut k: u64) -> Vec<Vec<u64>> {
    assert!(a.len() == a[0].len());
    let n = a.len();
    let mut r = vec![vec![0; n]; n];
    for i in 0..n {
        r[i][i] = 1;
    }
    while k > 0 {
        if k & 1 == 1 {
            r = mul(&r, &a);
        }
        a = mul(&a, &a);
        k >>= 1;
    }
    r
}

fn mul(a: &Vec<Vec<u64>>, b: &Vec<Vec<u64>>) -> Vec<Vec<u64>> {
    assert!(a.len() == a[0].len() && b.len() == b[0].len() && a.len() == b.len());
    let n = a.len();
    let mut r = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                let r = &mut r[i][j];
                *r += a[i][k] * b[k][j];
                *r %= M;
            }
        }
    }
    r
}
