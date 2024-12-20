fn main() {
    proconio::input! {s: [char; 3]}
    println!("{}", if s[0] == '<' {
        if s[1] == '<' {if s[2] == '<' {'B'} else {'C'}} else {'A'}
    } else {
        if s[1] == '<' {'A'} else {if s[2] == '<' {'C'} else {'B'}}
    });
}
