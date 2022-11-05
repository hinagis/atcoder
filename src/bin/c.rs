use superslice::Ext;
use itertools::Itertools;

fn main() {
    proconio::input!(mut p: [u32]);
    p.prev_permutation();
    println!("{}", p.iter().join(" "));
}
