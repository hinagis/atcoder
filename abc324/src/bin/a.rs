use itertools::Itertools;

fn main() {
    proconio::input! {a: [u32]}
    println!("{}", if a.iter().unique().count() == 1 {"Yes"} else {"No"});
}
