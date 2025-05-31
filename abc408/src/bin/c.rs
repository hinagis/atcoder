use proconio::{input as I, marker::Usize1 as U};

fn main() {
    I! {
        n: usize,
        m: usize
    }
    let mut f = vec![0i32; n + 1];
    for _ in 0..m {
        I! {
            l: U,
            r: usize
        }
        f[l] += 1;
        f[r] -= 1;
    }
    let mut ans = i32::MAX;
    let mut c= 0;
    for i in 0..n {
        c += f[i];
        ans = ans.min(c);
    }
    println!("{}", ans);
}
