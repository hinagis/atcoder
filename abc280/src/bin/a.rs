fn main() {
    proconio::input! {
        h: usize,
        _: usize,
        s: [String; h]
    }
    println!("{}", s.iter().fold(0, |t, s| t + s.chars().filter(|&c| c == '#').count()));
}
