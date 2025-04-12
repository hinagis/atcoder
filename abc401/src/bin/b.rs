use proconio::input as I;

fn main() {
    I! {n: usize}
    let mut c = 0;
    let mut f = false;
    for _ in 0..n {
        I! {s: String}
        match s.as_str() {
            "login" => f = true,
            "logout" => f = false,
            "private" if !f => c += 1,
            _ => ()
        }
    }
    println!("{}", c);
}
