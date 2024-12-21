fn main() {
    proconio::input! {a: [u64]}
    let mut q = std::collections::VecDeque::new();
    for a in a {
        let mut a = a;
        while let Some(p) = q.pop_back() {
            if a == p {
                a += 1;
            } else {
                q.push_back(p);
                break;
            }
        }
        q.push_back(a);
    }
    println!("{}", q.len());
}
