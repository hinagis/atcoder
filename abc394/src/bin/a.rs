fn main() {
    proconio::input! {s: String}
    println!("{}", vec!['2'; s.chars().filter(|&c| c == '2').count()].iter().collect::<String>());
}
