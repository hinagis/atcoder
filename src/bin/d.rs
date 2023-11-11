fn main() {
    proconio::input! {s: String}
    let mut q = std::collections::VecDeque::<(char, u32)>::with_capacity(2 * 10usize.pow(5));
    let mut f = 0;
    for c in s.chars() {
        if c == 'A' {
            f = 1;
        }
        if c == 'B' {
            f = if f == 1 {2} else {0};
        }
        if c == 'C' {
            if f == 2 {
                q.pop_back();
                q.pop_back();
                if let Some(&b) = q.back() {
                    f = b.1;
                }
                continue;
            } else {
                f = 0;
            }
        }
        q.push_back((c, f));
    }
    if q.len() == 0 {
        print!("");
    } else {
        println!("{}",  q.iter().map(|(c,_)| c).collect::<String>());
    }
}
