use proconio::{input as I, marker::Usize1 as U};

fn main() {
    I! {
        n: u32,
        m: usize,
        k: u32,
    }
    let mut o = vec![true; 2usize.pow(n)];
    for _ in 0..m {
        I! {
            c: usize,
            a: [U; c],
            r: char
        }
        let mut b = 0;
        for a in a {
            b |= 1 << a;
        }
        if r == 'o' {
            for i in 0..2usize.pow(n) {
                o[i] &= (b & i).count_ones() >= k;
            }
        } else {
            for i in 0..2usize.pow(n) {
                o[i] &= (b & i).count_ones() < k;
            }
        }
    }
    println!("{}", o.iter().filter(|&&o| o).count());
}
