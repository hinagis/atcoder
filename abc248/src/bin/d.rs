use proconio::{input as I, marker::Usize1 as U1};
fn main() {
    I! {
        n: usize,
        a: [U1; n],
        q: usize,
    }
    let mut c = vec![vec![]; n];
    for i in 0..n {
        c[a[i]].push(i + 1);
    }
    for _ in 0..q {
        I! {
            l: usize,
            r: usize,
            x: U1,
        }
        if c[x].len() > 0 {
            let l = match c[x].binary_search(&l) {
                Ok(x) => x,
                Err(x) => x,
            };
            let r = match c[x].binary_search(&r) {
                Ok(x) => x + 1,
                Err(x) => x,
            };
            println!("{}", r - l);
        } else {
            println!("0");
        }
    }
}
