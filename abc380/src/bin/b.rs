use itertools::Itertools;

fn main() {
    proconio::input! {s: String}
    println!("{}", s.trim_start_matches('|')
        .trim_end_matches('|')
        .split('|')
        .map(|s| s.len())
        .join(" ")
    );
}
