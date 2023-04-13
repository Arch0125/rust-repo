fn is_palindrome(s: &str) -> bool {
    let mut res = String::new();
    let l = s.len();
    for i in 0..l {
        match s.chars().nth(l - i - 1) {
            Some(c) => res.push(c),
            None => (),
        }
    }
    res == s
}

fn main(){
    let s = "racecar";
    println!("\"{}\" is a palindrome: {}", s, is_palindrome(s));
}