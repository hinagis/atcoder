fn main() {
    proconio::input! {
        n: usize,
        a: [u32; n]
    }
    let mut a = a.iter().enumerate().map(|(i, a)| (a, i)).collect::<Vec<(_, _)>>();
    a.sort();
    println!("{}", a[a.len() - 2].1 + 1);
}
