use proconio::input as I;

fn main() {
    I! {x: usize}
    let mut s = "HelloWorld".to_string();
    s.remove(x - 1);
    println!("{}", s);
}
