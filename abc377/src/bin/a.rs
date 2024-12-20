use itertools::Itertools;

fn main() {
    proconio::input! {mut s: String}
    println!("{}", if s.chars().sorted().collect::<String>() == "ABC" {"Yes"} else {"No"});
}
