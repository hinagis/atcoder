use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {
        n: usize,
        s: C
    }
    let mut d = vec![[i32::MIN; 3]; n + 1];
    d[0][0] = 0;
    d[0][1] = 0;
    d[0][2] = 0;
    for i in 0..n {
        match s[i] {
            'R' => {
                d[i + 1][0] = d[i + 1][0].max(d[i][1]).max(d[i][2]);
                d[i + 1][1] = d[i + 1][1].max(d[i][0] + 1).max(d[i][2] + 1);
            },
            'P' => {
                d[i + 1][1] = d[i + 1][1].max(d[i][0]).max(d[i][2]);
                d[i + 1][2] = d[i + 1][2].max(d[i][0] + 1).max(d[i][1] + 1);
            },
            _ => {
                d[i + 1][2] = d[i + 1][2].max(d[i][0]).max(d[i][1]);
                d[i + 1][0] = d[i + 1][0].max(d[i][2] + 1).max(d[i][1] + 1);
            },
        }
    }
    println!("{}", d[n].iter().max().unwrap());
}
