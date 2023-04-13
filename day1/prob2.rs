fn count_vowels(s: &str) -> usize {
    let mut count = 0;
    let length = s.len();
    for i in 0..length {
        match s.chars().nth(i) {
            Some('a') | Some('e') | Some('i') | Some('o') | Some('u') | Some('A') | Some('E') | Some('I') | Some('O') | Some('U')  => count += 1,
            _ => (),
        }
    }
    count
}

fn main(){
    let s = "Hello World";
    println!("There are {} vowels in \"{}\"", count_vowels(s), s);
}