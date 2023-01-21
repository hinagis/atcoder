use proconio::{input as I, marker::Bytes as B};

fn main() {
    I! {s: B}
    println!("{}",
        s.iter()
         .rev()
         .enumerate()
         .map(|(i, c)| (c - b'@') as u64 * 26u64.pow(i as u32))
         .sum::<u64>()
    );
}
