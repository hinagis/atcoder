use itertools::Itertools;

fn main() {
    proconio::input! {
        n: usize,
        mut a: [i32; n]
    }
    for i in 1..=n {
        if !a.contains(&(i as i32)) {
            let mut f = true;
            for j in 0..n {
                if a[j] == -1 {
                    f = false;
                    a[j] = i as i32;
                    break;
                }
            }
            if f {
                println!("No");
                return;
            }
        }
    }
    println!("Yes\n{}", a.iter().join(" "));
}
