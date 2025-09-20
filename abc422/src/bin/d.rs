use itertools::Itertools;

fn main() {
    proconio::input! {
        n: u32,
        k: u32
    }

    let s = 2u32.pow(n);
    let (m, r) = (k / s, k % s);
    let mut b = Vec::with_capacity(s as usize);
    for i in 0..s {
        b.push(m + (i.reverse_bits() >> (u32::BITS - n) < r) as u32);
    }
    println!("{}\n{}", (k % s != 0) as usize, b.iter().join(" "));
}
