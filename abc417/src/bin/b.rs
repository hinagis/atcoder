use itertools::Itertools;

fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        mut a: [u32; n],
        b: [u32; m],
    }
    for b in b {
        if let Ok(i) = a.binary_search(&b) {
            a.remove(i);
        }
    }
    println!("{}", a.iter().join(" "));
}
