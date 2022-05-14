fn main() {
    proconio::input! {_: usize}
    let mut a = Vec::with_capacity(300);
    for i in 1..=100 {
        a.push(i);
        a.push(i * 100);
        a.push(i * 10000);
    }

    println!("{}\n{}", a.len(), a.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(" "));
}
