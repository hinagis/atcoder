fn main() {
    proconio::input! {
        n: usize,
        mut a: [u32; n]
    }
    a.sort();

    let mut c = 0u64;
    let (mut i, mut j) = (0, 0);
    'outer: while i < n {
        while a[i] == a[j] {
            j += 1;
            if j >= n {
                break 'outer;
            }
        }
        c += ((j - i) as u64) * ((n - j) as u64);
        i = j;
    }
    println!("{}", c);
}
