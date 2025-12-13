use proconio::input as I;

fn main() {
    I! {
        n: usize,
        s: String
    }
    let o = if n > s.len() {n - s.len()} else {0};
    let o = std::iter::repeat('o').take(o);
    let o = o.collect::<String>();
    println!("{o}{s}");
}
