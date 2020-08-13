fn main() {
    proconio::input! {
        n: usize,
        a: [u64; n],
    }


    println!("{}", calc(a));
}

fn calc(a: Vec<u64>) -> u64 {
    let n = a.len();
    if n == 2 {
        a[0] + a[1]
    } else {
        let mut v = 0;
        for i in 0..(n - 1) {
            let mut a = a.clone();
            a[i] += a[i + 1];
            a.remove(i + 1);
            v = if v == 0 {
                a[i] + calc(a)
            } else {
                std::cmp::min(v, a[i] + calc(a))
            }
        }
        v
    }
}