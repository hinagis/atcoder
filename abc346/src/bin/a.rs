fn main() {
    proconio::input! {a: [u32]}
    println!("{}", a
        .iter()
        .skip(1)
        .zip(a.iter())
        .map(|(a, b)| (a * b).to_string())
        .collect::<Vec<_>>()
        .join(" "));
}
