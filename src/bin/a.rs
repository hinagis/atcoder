fn main() {
    proconio::input! {
        n: usize,
        h: [u32; n]
    }
    let mut m = 0;
    for i in 0..n {
        if h[i] > h[m] {
            m = i;
        }
    }

    println!("{}", m + 1);
}
