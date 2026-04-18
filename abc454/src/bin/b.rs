use proconio::{input as I, marker::Usize1 as U};

fn main() {
    I! {
        n: usize,
        m: usize,
        f: [U; n],
    }
    let mut c = vec![false; m];
    let mut d = false;
    for i in f {
        if c[i] {
            d = true;
        }
        c[i] = true;
    }
    println!("{}\n{}", if d {"No"} else {"Yes"}, if c.iter().all(|f| *f) {"Yes"} else {"No"});
}
