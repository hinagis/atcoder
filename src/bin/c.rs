use itertools::Itertools;

fn main() {
    proconio::input! {c: [[u32; 3]; 3]}
    let mut r = 0;
    for p in (0..9).permutations(9) {
        let mut x = vec![None; 3];
        let mut y = vec![None; 3];
        let mut z = vec![None; 2];

        let mut f = true;
        for p in p {
            let i = p / 3;
            let j = p % 3;

            if let Some(v) = x[j] {
                if v == c[i][j] {
                    f = false;
                    break;
                }
                x[j] = None;
            } else {
                x[j] = Some(c[i][j]);
            }
            if let Some(v) = y[i] {
                if v == c[i][j] {
                    f = false;
                    break;
                }
                y[i] = None;
            } else {
                y[i] = Some(c[i][j]);
            }
            if i == j {
                if let Some(v) = z[0] {
                    if v == c[i][j] {
                        f = false;
                        break;
                    }
                    z[0] = None;
                } else {
                    z[0] = Some(c[i][j]);
                }
            }
            if i == 2 - j {
                if let Some(v) = z[1] {
                    if v == c[i][j] {
                        f = false;
                        break;
                    }
                    z[1] = None;
                } else {
                    z[1] = Some(c[i][j]);
                }
            }
        }
        if f {
            r += 1;
        }
    }
    println!("{}", r as f64 / 362880f64);
}
