use proconio::input as I;

fn main() {
    I! {s: String}
    println!("{}", s.chars().filter(|&c| c == 'i' || c == 'j').count());
}
