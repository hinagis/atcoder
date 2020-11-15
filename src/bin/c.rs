use itertools::Itertools;

fn main() {
    proconio::input! {
        n: usize,
        k: u32,
        t: [[u32; n]; n],
    }
    let p = (1..n).permutations(n - 1);
    let mut r = 0;
    for p in p {
        let mut sum = 0;
        let mut pre = 0;
        for i in 0..(n - 1) {
            sum += t[pre][p[i]];
            pre = p[i];
        }
        sum += t[pre][0];
        if sum == k {r += 1};
    }

    println!("{}", r);
}
