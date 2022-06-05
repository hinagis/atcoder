fn main() {
    proconio::input! {n: usize}

    let mut r = 0;
    for i in 1..=n {
        let mut k = i;
        let mut j = 2;
        while j * j <= k {
            while k % (j * j) == 0 {
                k /= j * j;
            }
            j += 1;
        }
        let mut j = 1;
        while k * j * j <= n {
            r += 1;
            j += 1;
        }
    }
    println!("{}", r);
}
