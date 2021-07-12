use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[u32; w]; h]
    }
    let mut vs = vec![0; w];
    let mut hs = vec![0; h];
    for i in 0..h {
        for j in 0..w {
            hs[i] += a[i][j];
            vs[j] += a[i][j];
        }
    }

    for i in 0..h {
        print!("{}", hs[i] + vs[0] - a[i][0]);
        for j in 1..w {
            print!(" {}", hs[i] + vs[j] - a[i][j]);
        }
        println!("");
    }
}
