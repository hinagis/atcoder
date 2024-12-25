fn main() {
    proconio::input! {mut n: u64}
    let mut q = std::collections::VecDeque::new();
    while n > 0 {
        if n % 2 == 0 {
            n /= 2;
            q.push_front('B');
        } else {
            n -= 1;
            q.push_front('A');
        }
    }

    println!("{}", q.iter().collect::<String>());
}
