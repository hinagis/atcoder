fn main() {
    proconio::input! {
        mut n: u32,
        k: usize,
    }

    for _ in 0..k {
        let mut g1: Vec<char> = n.to_string().chars().collect();
        g1.sort();
        let mut g2 = g1.clone();
        g2.reverse();
        let g1: String = g1.iter().collect();
        let g2: String = g2.iter().collect();
        let g1: u32 = g1.parse().unwrap();
        let g2: u32 = g2.parse().unwrap();
        n = g2 - g1;
    }

    println!("{}", n);
}
