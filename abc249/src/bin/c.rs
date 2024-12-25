use proconio::{input as I, marker::Bytes as B};

fn main() {
    I! {
        n: usize,
        k: u32,
        s: [B; n]
    }

    println!("{}", dfs(&s, vec![0; 26], n, k, 0));
}

fn dfs(s: &Vec<Vec<u8>>, c: Vec<u32>, n: usize, k: u32, i: usize) -> u32 {
    let mut m = 0;
    for i in i..n {
        let mut c = c.clone();
        for &s in &s[i] {
            c[(s - b'a') as usize] += 1;
        }
        m = m.max(c.iter().filter(|&&c| c == k).count() as u32);
        m = m.max(dfs(s, c, n, k, i + 1));
    }
    m
}
