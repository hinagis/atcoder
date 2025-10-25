fn main() {
    proconio::input! {
        n: usize,
        m: u32,
        a: [u32; n]
    }
    let s = a.iter().sum::<u32>();
    for a in a {
        if s - a == m {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
