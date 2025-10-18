use proconio::{input as I, marker::Usize1 as U};

fn main() {
    I! {
        n: u32,
        e: [(U, U)]
    }
    let mut m = u32::MAX;
    for i in 0..2u32.pow(n) {
        let mut c = 0;
        for (u, v) in &e {
            if (i & (1 << u) == 0) == (i & (1 << v) == 0) {
                c += 1;
            }
        }
        m = m.min(c);
    }
    println!("{}", m);
}
