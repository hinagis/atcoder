fn main() {
    proconio::input! {
        n: usize,
        a: [i64; n]
    }
    let mut c = vec![0; 200];

    for i in 0..200 {
        for j in 0..n {
            if a[j] % 200 == i as i64 {
                c[i] += 1;
            }
        }
    }
    println!("{}", c.iter().fold(0, |s, &c| s + if c >= 2 {comb(c, 2)} else {0}));
}

fn comb(n: i64, r: i64) -> i64 {
    (0..n + 1)
        .rev()
        .zip(1..r + 1)
        .fold(1, |mut r, (n, d)| { r *= n; r /= d; r })
}
