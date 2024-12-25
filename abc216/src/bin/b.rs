fn main() {
    proconio::input! {
        n: usize,
        st: [(String, String); n]
    }
    for i in 0..n {
        let (s, t) = st[i].clone();
        for j in i+1..n {
            let (u, v) = st[j].clone();
            if s == u && t == v {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
