fn main() {
    proconio::input! {
        n: usize,
        l: [u8; n]
    }
    let mut a = 0;
    while a < n && l[a] == 0 {
        a += 1;
    }
    let mut b = n;
    while b > 0 && l[b - 1] == 0 {
        b -= 1;
    }
    println!("{}", if a > b {0} else {b - 1 - a});
}
