fn main() {
    proconio::input! { mut n: u64, }

    let mut r = String::new();
    while n > 0 {
        n -= 1;
        r.insert(0, ((n % 26) as u8 + b'a') as char);
        n /= 26;
    }
    println!("{}", r);
}
