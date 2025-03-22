fn main() {
    proconio::input! {a: [usize; 7]}
    let mut c = vec![0; 13];
    for i in 0..7 {
        c[a[i] - 1] += 1;
    }
    c.sort_by(|a, b| b.cmp(&a));
    println!("{}", if c[0] >= 3 && c[1] >= 2 {"Yes"} else {"No"});
}
