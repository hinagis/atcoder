use proconio::input;

fn main() {
    input! {
        mut s: String,
    }
    s.truncate(3);
    println!("{}", s);
}
