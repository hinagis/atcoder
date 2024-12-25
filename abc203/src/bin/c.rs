fn main() {
    proconio::input! {
        n: usize,
        mut k: u64,
        mut ab: [(u64, u64); n],
    }
    ab.sort_by(|(ai, _), (aj, _)| ai.cmp(aj));

    for &(a, b) in &ab {
        if k < a {
            break;
        } else {
            k += b;
        }
    }
    println!("{}", k);
}
