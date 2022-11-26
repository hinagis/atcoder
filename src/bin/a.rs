fn main() {
    proconio::input! {s: String}
    let w = s.chars().filter(|&c| c == 'w').count();
    let v = s.len() - w;
    println!("{}", v + w * 2);
}
