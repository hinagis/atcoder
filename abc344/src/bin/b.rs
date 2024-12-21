use itertools::Itertools;

fn main() {
    let mut r = vec![];
    loop {
        proconio::input! {a: u32}
        r.push(a);
        if a == 0 {break;}
    }
    println!("{}", r.iter().rev().join("\n"));
}
