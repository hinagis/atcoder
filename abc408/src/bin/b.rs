use itertools::Itertools;

fn main() {
    proconio::input! {
        n: usize,
        mut a: [u32; n]
    }
    a.sort();
    let a = a.iter().unique().collect::<Vec<_>>();
    println!("{}\n{}", a.len(), a.iter().join(" "));
}
