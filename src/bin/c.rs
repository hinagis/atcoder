use proconio::input as I;

fn main() {
    I! {n: usize, q: usize}
    let mut a = (1..=n).map(|i| i).collect::<Vec<usize>>();
    let mut h = std::collections::HashMap::new();
    for i in 1..=n {
        h.insert(i, i - 1);
    }
    for _ in 0..q {
        I!{x: usize}
        let i = *h.get(&x).unwrap();
        let j = if i == n - 1 {i - 1} else {i + 1};
        *h.get_mut(&x).unwrap() = j;
        *h.get_mut(&a[j]).unwrap() = i;
        a.swap(i, j);
    }

    println!("{}", a.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(" "));
}
