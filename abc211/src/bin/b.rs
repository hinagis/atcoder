fn main() {
    proconio::input! {mut s: [String; 4]}
    s.sort();
    println!("{}", if s == ["2B", "3B", "H", "HR"] {"Yes"} else {"No"});
}
