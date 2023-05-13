fn main() {
    proconio::input! {
        n: usize,
        a: [u32; n]
    }
    let mut b = Vec::with_capacity(10000);
    for i in 0..n - 1 {
        if a[i] < a[i + 1] {
            for j in a[i]..a[i + 1] {
                b.push(j);
            } 
        } else {
            for j in (a[i + 1] + 1..=a[i]).rev() {
                b.push(j);
            } 
        }
    }
    b.push(a[n - 1]);
    println!("{}", b.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(" "));
}
