fn main() {
    proconio::input! {
        h: usize,
        w: usize,
        a: [[u8; w]; h]
    }

    println!("{}", a.iter()
        .map(|a| a.iter()
            .map(|&a| if a == 0 {'.'} else {(a - 1 + b'A') as char})
            .collect::<String>()
        )
        .collect::<Vec<_>>()
        .join("\n")
    );
}
