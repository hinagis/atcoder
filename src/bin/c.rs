use itertools::Itertools;

fn main() {
    proconio::input! {
        h: usize,
        w: usize,
        k: usize,
        s: [String; h],
    }
    let mut r = 0;

    let mut c = vec![vec!['.'; w]; h];
    for i in 0..h {
        let si: Vec<char> = s[i].chars().collect();
        for j in 0..w {
            c[i][j] = si[j];
        }
    }
    for i in 0..h {
        let i_s = (0..h).combinations(i);
        for ii in i_s {
            let mut c = c.clone();
            for iii in ii {
                for j in 0..w {
                    c[iii][j] = '.';
                }
            }
            for j in 0..w {
                let j_s = (0..w).combinations(j);
                for jj in j_s {
                    let mut c = c.clone();
                    for jjj in jj {
                        for i in 0..h {
                            c[i][jjj] = '.';
                        }
                    }
                    let mut cn = 0;
                    for i in 0..h {
                        for j in 0..w {
                            if c[i][j] == '#' {
                                cn += 1;
                            }
                        }
                    }
                    if cn == k {
                        r += 1;
                    }
                }
            }
        }
    }
    println!("{}", r);
}
