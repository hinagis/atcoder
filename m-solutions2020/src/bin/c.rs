fn main() {
    proconio::input! {
        n: usize,
        mut k: usize,
        a: [u64; n],
    }

    k -= 1;
    for i in (k + 1)..n {
        if a[i] > a[i - k - 1] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
