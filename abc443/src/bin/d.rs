use proconio::input as I;

fn main() {
    I! {t: u32}
    for _ in 0 .. t {
        I! {
            n: usize,
            r: [u64; n]
        }
        let mut c = r.clone();
        for i in 1..n {
            c[i] = c[i].min(c[i - 1] + 1);
        }
        for i in (1..n).rev() {
            c[i - 1] = c[i - 1].min(c[i] + 1);
        }
        println!("{}", (0..n).map(|i| r[i] - c[i]).sum::<u64>());
    }
}
