use proconio::input as I;

fn main() {
    I! {mut a: [u32; 3]}
    a.sort();
    println!("{}{}{}", a[2], a[1], a[0]);
}
