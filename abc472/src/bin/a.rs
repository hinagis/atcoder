use proconio::input as I;

fn main() {
    I! {s: String}
    println!("{}", s.replace(|c| c != 'A', "."));
}
