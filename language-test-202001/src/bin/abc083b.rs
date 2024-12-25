fn main() {
    proconio::input! {
        n: usize,
        a: usize,
        b: usize,
    }
    let mut r = 0;
    for i in 1..=n {
        let mut sum = 0;
        for c in i.to_string().chars() {
            sum += (c as u8 - '0' as u8) as usize;
        }
        if sum >= a && sum <= b {
            r += i;
        }
    }
    println!("{}", r);
}
