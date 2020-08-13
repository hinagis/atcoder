const M: u64 = 1000_000_007;

fn main() {
    proconio::input! {
        n: usize,
        a: [[u8; n]; n],
    }

    println!("{}", calc(0, n, a));
}

fn calc(i: usize, n: usize, a: Vec<Vec<u8>>) -> u64 {
    if i == n {
        1
    } else {
        let mut v = 0;
        for j in 0..n {
            if a[i][j] == 1 {
                let mut a = a.clone();
                for k in 0..n {
                    a[k][j] = 0;
                }
                v += calc(i + 1, n, a) % M;
            }
        }
        v
    }
}
