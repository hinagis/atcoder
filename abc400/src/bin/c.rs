use num_integer::Roots;

fn main() {
    proconio::input! {n: u64}
    let mut s = 0;
    let mut a = 2;
    while a <= n {
        let m = n / a;
        s += (m.sqrt() + 1) / 2;
        a *= 2;
    }
    println!("{}", s);
}
