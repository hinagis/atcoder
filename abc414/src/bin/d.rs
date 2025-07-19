use itertools::Itertools;

fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        mut x: [u64; n]
    }
    x.sort();
    println!("{}", x.windows(2).map(|e| e[1] - e[0]).sorted().take(n - m).sum::<u64>());
}
