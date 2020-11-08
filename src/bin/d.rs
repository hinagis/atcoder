fn main() {
    proconio::input! {
        n: usize,
        a: [i64; n]
    }
    let mut mp = 0;
    let mut p = 0;
    let mut sum = 0;
    let mut posi = 0;
    for i in 0..n {
        sum += a[i];
        posi = posi.max(sum);
        mp = mp.max(p + posi);
        p += sum;
    }

    println!("{}", mp);
}
