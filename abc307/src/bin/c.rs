use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {
        ha: usize, wa: usize,
        a: [C; ha],
        hb: usize, wb: usize,
        b: [C; hb],
        hx: usize, wx: usize,
        x: [C; hx],
    }

    for ta in 0..20 {
        for la in 0..20 {
            let mut fx = vec![vec![false; wx]; hx];
            for i in 0..hx {
                for j in 0..wx {
                    if x[i][j] == '#' {
                        fx[i][j] = true;
                    }
                }
            }
            let mut f = true;
            for i in 0..ha {
                for j in 0..wa {
                    if a[i][j] == '#' {
                        if ta + i < 10 || la + j < 10 ||
                            ta + i - 10 >= hx || la + j - 10 >= wx ||
                            x[ta + i - 10][la + j - 10] != '#' {
                            f = false;
                        } else {
                            fx[ta + i - 10][la + j - 10] = false;
                        }
                    }
                }
            }
            if f {
                for tb in 0..20 {
                    for lb in 0..20 {
                        let mut fb = f;
                        let mut fxb = fx.clone();
                        for i in 0..hb {
                            for j in 0..wb {
                                if b[i][j] == '#' {
                                    if tb + i < 10 || lb + j < 10 ||
                                        tb + i - 10 >= hx || lb + j - 10 >= wx ||
                                        x[tb + i - 10][lb + j - 10] != '#' {
                                        fb = false;
                                    } else {
                                        fxb[tb + i - 10][lb + j - 10] = false;
                                    }
                                }
                            }
                        }
                        if fb && fxb.iter().fold(0, |s, f| s + f.iter().filter(|&&f| f).count()) == 0{
                            println!("Yes");
                            return;
                        }
                    }
                }
            }
        }
    }
    println!("No");
}
