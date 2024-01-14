fn main() {
    proconio::input! {mut n: usize}
    n -= 1;
    let e = ['0', '2', '4', '6', '8'];
    let mut s = String::new();
    if n == 0 {
        s.push('0');
    } else {
        while n > 0 {
            s.push(e[n % 5]);
            n /= 5;
        }
    }
    println!("{}", s.chars().rev().collect::<String>());
}
