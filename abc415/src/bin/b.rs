use itertools::Itertools;

fn main() {
    proconio::input! {s: String}
    let v =         s.chars()
         .enumerate()
         .filter(|&(_, c)| c == '#')
         .map(|(i, _)| i + 1)
         .collect_vec();
    println!("{}", (0..v.len()).step_by(2).map(|i| v[i].to_string() + "," + &v[i + 1].to_string()).join("\n"));
}
