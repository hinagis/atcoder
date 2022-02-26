use proconio::input as I;

fn main() {
    I! {
        n: usize,
        m: usize,
    }

    let mut c = std::collections::HashMap::new();
    for _ in 0..n {
        I! {a: u32}
        *c.entry(a).or_insert(0) += 1;
    }
    for _ in 0..m {
        I! {b: u32}
        if let Some(p) = c.get_mut(&b) {
            if *p == 0 {
                println!("No");
                return;
            } else {
                *p -= 1;
            }
        } else {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
