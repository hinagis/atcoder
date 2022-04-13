fn main() {
    proconio::input! {n: usize}
    println!("{}", calc(n).iter().map(|i| i.to_string()).collect::<Vec<_>>().join(" "));
}

fn calc(n: usize) -> Vec<usize> {
    if n == 1 {
        vec![1]
    } else {
        let c = calc(n - 1);
        vec![c.clone(), vec![n], c].concat()
    }
}