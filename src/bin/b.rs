use proconio::input;

fn main() {
    input! {n: usize, x: u32}
    let mut a = vec![];
    for _ in 0..n {
        input!{ai: u32}
        if ai != x {
            a.push(ai.to_string());
        }
    }
    println!("{}", a.join(" "));
}
