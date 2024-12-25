use itertools::Itertools;

fn main() {
    proconio::input! {
       n:usize,
       xy:[(i32, i32);n],
    }

    let mut v = std::collections::HashSet::new();
    for p in xy.iter().permutations(2) {
        let x = p[0].0 - p[1].0;
        let y = p[0].1 - p[1].1;
        let d = num_integer::gcd(x, y);
        v.insert((x / d, y / d));
    }
    println!("{}", v.len());
}
