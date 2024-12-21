fn main() {
    proconio::input! {
        _: usize,
        k: usize,
        a: [u32; k]
    }
    let mut o = vec![0; k + 1];
    let mut e = vec![0; k + 1];
    for i in 1..k {
        if i % 2 == 0 {
            e[i] += a[i] - a[i - 1];
        } else {
            o[i] += a[i] - a[i - 1];
        }
        e[i + 1] = e[i];
        o[i + 1] = o[i];
    }
    let mut m = std::u32::MAX;
    if k % 2 == 0 {
        m = o[k];
    } else {
        for i in 0..k {
            let l = if i > 1 {o[i - 1]} else {0};
            let r = if i < k - 1 {e[k] - e[i + 1]} else {0};
            m = m.min(l + r + if i % 2 != 0 && i < k - 1 {a[i + 1] + a[i - 1]} else {0});
        }
    }
    println!("{}", m);
}
