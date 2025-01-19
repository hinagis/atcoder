fn main() {
    proconio::input! {mut x: u64}
    for i in 2.. {
        if i == x {break;}
        x /= i;
    }
    println!("{}", x);
}
