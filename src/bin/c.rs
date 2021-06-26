fn main() {
    proconio::input! {
        n: usize,
        tlr: [(u8, u32, u32); n]
    }
    let mut c = 0;
    for i in 0..n {
        let (ti, li, ri) = tlr[i];
        for j in i + 1..n {
            let (tj, lj, rj) = tlr[j];
            if (ri == lj && (ti == 1 || ti == 3) && (tj == 1 || tj == 2))
            || (li == rj && (ti == 1 || ti == 2) && (tj == 1 || tj == 3))
            || (li <= lj && lj < ri)
            || (li < rj && rj <= ri)
            || (li <= lj && rj <= ri)
            || (lj <= li && ri <= rj) {
                c += 1;
            }
        }
    }

    println!("{}", c);
}
