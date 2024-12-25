fn main() {
    proconio::input! {
        n: usize,
        mut a: [i128; n]
    }
    a.sort();

    let mut r = 0;
    /*
    for i in 0..(n - 1) {
        for j in (i + 1)..n {
            r += (a[i] -a[j]).abs();
        }
    }
    */
    for i in 0..n {
        r += a[i] * i as i128;
        r -= a[i] * (n - 1 - i) as i128;
    }

    println!("{}", r);
}
