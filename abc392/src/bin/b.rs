use itertools::Itertools;

fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        mut a: [usize; m]
    }
    a.sort();
    let mut x = Vec::with_capacity(n);
    for i in 1..=n {
        if !a.contains(&i) {
            x.push(i);
        }
    }
    println!("{}\n{}", x.len(), x.iter().join(" "));
}
