use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {
        h: usize,
        w: usize,
        s: [C; h]
    }
    let mut f = vec![vec![u32::MAX; w]; h];
    f[0][0] = 0;
    let mut a = vec![vec![]; 26];
    for i in 0..h {
      for j in 0..w {
        if s[i][j] != '.' && s[i][j] != '#' {
          a[(s[i][j] as u8 - 'a' as u8) as usize].push((i, j));
        }
      }
    }
    let mut q = std::collections::VecDeque::new();
    q.push_back((0, 0, 0));
    while let Some((i, j, c)) = q.pop_front() {
      let c = c + 1;
      if i > 0 {
        let i = i - 1;
        if s[i][j] != '#' && c < f[i][j] {
          f[i][j] = c;
          q.push_back((i, j, c));
        }
      }
      if i < h - 1 {
        let i = i + 1;
        if s[i][j] != '#' && c < f[i][j] {
          f[i][j] = c;
          q.push_back((i, j, c));
        }
      }
      if j > 0 {
        let j = j - 1;
        if s[i][j] != '#' && c < f[i][j] {
          f[i][j] = c;
          q.push_back((i, j, c));
        }
      }
      if j < w - 1 {
        let j = j + 1;
        if s[i][j] != '#' && c < f[i][j] {
          f[i][j] = c;
          q.push_back((i, j, c));
        }
      }
      if s[i][j] != '.' {
        for &(u, v) in &a[(s[i][j] as u8 - 'a' as u8) as usize] {
          if u != i || v != j {
            let i = u;
            let j = v;
            if s[i][j] != '#' && c < f[i][j] {
              f[i][j] = c;
              q.push_back((i, j, c));
            }
          }
        }
        a[(s[i][j] as u8 - 'a' as u8) as usize] = vec![];
      }
    }

    if f[h - 1][w - 1] < u32::MAX {
      println!("{}", f[h - 1][w - 1]);
    } else {
      println!("-1");
    }
}
