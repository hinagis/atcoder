use proconio::{input as I};

fn main() {
    I! {n: usize}
    let mut r = 0;
    let mut a = vec![vec![]; 2 * n - 1];
    for i in 0..2 * n - 1 {
        r |= 1 << i;
        a[i] = vec![0; 2 * n - 1 - i];
        for j in 0..2 * n - 1 - i {
            I! {b: u32}
            a[i][j] = b;
        }
    }
    r |= 1 << (2 * n - 1);
    let mut b = vec![];
    dfs(&a, &mut b, 0, n, r, 0);
    let mut m = 0;
    for &b in &b {
        m = m.max(b)
    }
    println!("{}", m);
}

fn dfs(a: &Vec<Vec<u32>>, b: &mut Vec<u32>, c: u32, n: usize, r: usize, i: usize) {
    if r == 0 {
        b.push(c)
    } else {
        let r = r & !(1 << i);
        let s = calc(r, n, i, 0);
        let p = 1 << (i + 1 + s);
        if r & p  != 0 {
            dfs(a, b, c ^ a[i][s], n, r & !p, i + 1 + calc(r, n, i, s + 1));
        }
        for j in s + 1..2 * n - 1 - i {
            let p = 1 << (i + 1 + j);
            if r & p != 0 {
                dfs(a, b, c ^ a[i][j], n, r & !p, i + 1 + s)
            }
        }
    }
}

fn calc(r: usize, n: usize, i: usize, mut s: usize) -> usize {
    while s < 2 * n - 1 && r & (1 << (i + 1 + s)) == 0 {
        s += 1
    }
    s
}