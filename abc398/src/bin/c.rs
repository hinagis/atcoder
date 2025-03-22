fn main() {
    proconio::input! {
        n: usize,
        a: [usize; n]
    }
    let mut a = a.iter().enumerate().collect::<Vec<_>>();
    a.sort_by(|a, b| b.1.cmp(&a.1));
    let mut i = 0;
    while i < n {
        let mut k = i;
        for j in i + 1..n {
            if a[i].1 != a[j].1 {break}
            k = j;
        }
        if k == i {
            println!("{}", a[k].0 + 1);
            return;
        }
        i = k + 1;
    }

    println!("-1");
}
