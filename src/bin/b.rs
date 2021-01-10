fn main() {
    proconio::input! {
        n: usize,
        a: [i32; n],
        b: [i32; n],
    }
    let mut s = 0;
    for i in 0..n {
        s += a[i] * b[i];
    }

    println!("{}", if s == 0 {"Yes"} else {"No"});
}
