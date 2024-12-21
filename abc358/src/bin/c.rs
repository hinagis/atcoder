fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        s: [String; n],
    }
    let mut b = vec![0; n];
    let mut d = vec![vec![false; m]; n];
    for i in 0..n {
        for (j, c) in s[i].chars().enumerate() {
            if c == 'o' {
                b[i] |= 1 << j;
                d[i][j] = true;
            }
        }
    }
    
    println!("{}", calc(&d, &b, n, m, 0, (0, 0)));
}

fn calc(d: &Vec<Vec<bool>>, b: &Vec<u32>, n: usize, m: usize, j: usize, c: (u32, u32)) -> u32 {
    if j < m {
        if c.0 & (1 << j) == 0 {
            let mut r = u32::MAX;
            for i in 0..n {
                if d[i][j] {
                    r = r.min(calc(d, b, n, m, j + 1, (c.0 | b[i], c.1 + 1)));
                }
            }
            r
        } else {
            calc(d, b, n, m, j + 1, c)
        }
    } else {
        c.1
    }
}
