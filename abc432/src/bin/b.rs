use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {mut x: C}
    x.sort();
    if x[0] == '0' {
        let z = x.iter().filter(|&&c| c == '0').count();
        println!("{}{}{}",
            x[z],
            (0..z).map(|_| '0').collect::<String>(),
            x[z + 1..].iter().collect::<String>()
        );
    } else {
        println!("{}", x.iter().collect::<String>());
    }
}
