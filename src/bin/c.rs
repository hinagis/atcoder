fn main() {
    proconio::input! {
        n: usize,
        a: [i64; n]
    }


    let mut x = 0;
    for i in 0..n {
        x += a[i].pow(2);
    }
    x *= n as i64 - 1;

    let mut sum = 0;
    for i in 0..n {
        sum += a[i];
    }

    let mut y = 0;
    for i in 0..(n - 1) {
        sum -= a[i];
        y += a[i] * sum;
    }
    y *= 2;

    println!("{}", x - y);
}
