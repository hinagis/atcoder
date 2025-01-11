fn main() {
    proconio::input! {
        n: usize,
        a: [u64; n]
    }
    let mut c = 0;
    let mut j = 0;
    for i in 1..n {
        while a[j] <= a[i] / 2 {
            j += 1;
        }
        c += j;
    }
    println!("{}", c);
}
