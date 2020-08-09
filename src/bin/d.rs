fn main() {
    proconio::input! {
        n: usize,
        mut a: [u64; n],
    }

    a.sort();

    let mut r = 0;
    for i in 1..n {
        r += a[n - 1 - (i / 2)];
    }
    println!("{}", r);

//    a.reverse();
//    println!("{}", calc1(n, a));
}

fn _calc1(n: usize, a: Vec<u64>) -> u64 {
    let mut dic = vec![(a[0], 0u64)];
    let mut pre = 0;
    for i in 0..n-1 {
        if dic[pre].0 == a[i] {
            dic[pre].1 += 1;
        } else {
            dic.push((a[i], 1));
            pre += 1;
        }
    }

    let mut r = 0;
    for dici in dic {
        let n = if dici.1 >= 1 { dici.1 } else { 1 };
        r += dici.0 * n;
    }
    r
}
