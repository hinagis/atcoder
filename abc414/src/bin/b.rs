use proconio::input as I;

fn main() {
    I! {n: u32}
    let mut t = 0;
    let mut s = String::new();
    for _ in 0..n {
        I! {
            c: char,
            l: u64
        }
        t += l;
        if t > 100 {
            println!("Too Long");
            return;
        }
        for _ in 0..l {
            s.push(c);
        }
    }
    println!("{}", s);
}
