use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [u32; n],
        txy: [(u8, usize, usize); q]
    }
    let mut r = 0;
    for &(t, x, y) in &txy {
        if t == 1 {
            a.swap((x - 1 + r) % n, (y - 1 + r) % n);
        } else if t == 2 {
            r = (n + r - 1) % n;
        } else {
            println!("{}", a[(x - 1 + r) % n]);
        }
    }
}
