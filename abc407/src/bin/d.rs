fn main() {
    proconio::input! {
        h: usize,
        w: usize,
        a: [[u64; w]; h]
    }
    let mut t = 0;
    for i in 0..h {
        for j in 0..w {
            t ^= a[i][j];
        }
    }
    println!("{}", calc(&a, &mut vec![vec![true; w]; h], t));
}

fn calc(a: &Vec<Vec<u64>>, f: &mut Vec<Vec<bool>>, t: u64) -> u64 {
    let mut m = t;
    for i in 0..a.len() {
        for j in 0..a[i].len() {
            if i < a.len() - 1 && f[i][j] && f[i + 1][j] {
                let mut f = f.clone();
                f[i][j] = false;
                f[i + 1][j] = false;
                m = m.max(calc(a, &mut f, t ^ a[i][j] ^ a[i + 1][j]));
            }
            if j < a[i].len() - 1 && f[i][j] && f[i][j + 1] {
                let mut f = f.clone();
                f[i][j] = false;
                f[i][j + 1] = false;
                m = m.max(calc(a, &mut f, t ^ a[i][j] ^ a[i][j + 1]));
            }
        }
    }
    m
}
