const M: usize = 1000000000;

fn main() {
    proconio::input! {
        n: usize,
        k: usize,
    }
    if k > n {
        println!("1");
        return;
    }
    let mut a = vec![1; n + 1];
    a[k] = k;
    for i in k + 1..=n {
        a[i] = (2 * a[i - 1] + M - a[i - 1 - k]) % M;
    }
    println!("{}", a[n]);
}
