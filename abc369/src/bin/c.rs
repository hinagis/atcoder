fn main() {
    proconio::input! {
        n: usize,
        a: [i64; n]
    }
    let mut c = 0;
    let mut l = 0;
    let mut r = 1;
    while r < n {
        if a[r] == a[r - 1] + a[l + 1] - a[l] {
            r += 1;
        } else {
            let n = r - l;
            c += (n + 1) * n / 2 - 1;
            l = r - 1;
        }
    }
    let n = r - l;
    c += (n + 1) * n / 2;
    println!("{}", c);
}
