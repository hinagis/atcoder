fn main() {
    proconio::input! {s: String}
    let u = s.chars().filter(|c| c.is_ascii_uppercase()).count();
    let l = s.chars().filter(|c| c.is_ascii_lowercase()).count();
    println!("{}", if u > l {s.to_uppercase()} else {s.to_lowercase()});
}
