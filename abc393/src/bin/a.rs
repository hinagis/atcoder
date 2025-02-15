fn main() {
    proconio::input! {s: [String; 2]}
    println!("{}", if s[0] == "sick" {if s[1] == "sick" {1} else {2}} else if s[1] == "sick" {3} else {4});
}
