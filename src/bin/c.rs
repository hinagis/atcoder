fn main() {
    proconio::input! {k: usize}
    let mut b = vec![];
    for i in 0..1 << 10 {
        let mut a = 0u64;
        for j in (0..10).rev() {
            if i >> j & 1 == 1 {
                a *= 10;
                a += j;
            }
        }
        b.push(a);
    }
    b.sort();
    println!("{}", b[k + 1]);
}
