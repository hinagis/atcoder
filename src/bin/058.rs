const M: usize = 100000;

fn main() {
    proconio::input! {mut n: usize, k: u64}

    let mut c = 0;

    let mut d = vec![M as u64; M];
    while c < k && d[n] >= M as u64 {
        d[n] = c;
        n = calc(n);
        c += 1;
    }

    if c < k {
        let s = d[n];
        let d = c - s;
        let r = (k - s) / d;
        c += (r - 1) * d;
    }

    while c < k {
        n = calc(n);
        c += 1;
    }

    println!("{}", n);
}

fn calc(n: usize) -> usize {
    const T: [usize; 5] = [1, 10, 100, 1000, 10000];
    (n + T.iter().fold(0, |s, k| s + ((n % (k * 10)) / k))) % M
}