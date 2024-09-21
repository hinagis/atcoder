use itertools::Itertools;

fn main() {
    proconio::input! {m: u64}
    let mut a = vec![];
    let mut s = 0;
    for _ in 0..20 {
        a.push({
            let mut a: u32 = 10;
            for b in 1..=10 {
                if s + 3u64.pow(b) > m {
                    a = b - 1;
                    break;
                }
            }
            s += 3u64.pow(a);
            a
        });
        if s == m {
            break;
        }
    }
    println!("{}\n{}", a.len(), a.iter().join(" "));
}
