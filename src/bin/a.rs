fn main() {
    proconio::input! {
        mut v: i32,
        a: i32,
        b: i32,
        c: i32,
    }
    v %= a + b + c;
    println!("{}", if v - a < 0 {'F'} else if v - a - b < 0 {'M'} else {'T'});
}
