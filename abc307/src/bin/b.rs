fn main() {
    proconio::input! {
        n: usize,
        s: [String; n]
    }
    for i in 0..n {
        for j in 0..n {
            if i != j {
                let a = format!("{}{}", s[i], s[j]);
                let b = a.clone().chars().rev().collect::<String>();
                if a == b {
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}
