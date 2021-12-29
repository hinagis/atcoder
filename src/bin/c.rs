use proconio::input;

fn main() {
    input! {
        n: usize,
        x: u128,
    }
    let mut a = vec![vec![]; n];
    for i in 0..n {
        input! {
            l: usize,
            t: [u64; l]
        }
        a[i] = t;
    }

    println!("{}", calc(&a, n, x, 0, 1));
}

fn calc(a: &Vec<Vec<u64>>, n: usize, x: u128, i: usize, v: u128) -> u32 {
    if i >= n {
        if v == x {1} else {0}
    } else {
        let mut c = 0;
        for j in 0..a[i].len() {
            c += calc(a, n, x, i + 1, v * a[i][j] as u128)
        }
        c
    }
}
