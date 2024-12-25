fn main() {
    proconio::input! {mut s: String}
    let mut t = vec![String::new(); s.len()];
    for i in 0..s.len() {
        t[i] = s.clone();
        let c = s.remove(0);
        s.push(c);
    }
    t.sort();
    println!("{}", t[0]);
    println!("{}", t[t.len() - 1]);
}
