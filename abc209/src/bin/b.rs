fn main() {
    proconio::input! {
        n: usize,
        x: u32,
        a: [u32; n]
    }
    let mut s = 0;
    for i in 0..n {
        s += a[i] - if i % 2 == 0 {0} else {1};
    }

    println!("{}", if x >= s {"Yes"} else {"No"});
}
