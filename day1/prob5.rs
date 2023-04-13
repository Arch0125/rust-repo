fn reverse_words(s: &str) -> String {
    let mut result = String::new();
    let mut word = String::new();
    let length = s.len();
    for i in 0..length {
        match s.chars().nth(i) {
            Some(' ') => {
                result.push_str(&word);
                result.push(' ');
                word = String::new();
            },
            Some(c) => word.insert(0, c),
            None => (),
        }
    }
    result.push_str(&word);
    result
}

fn main(){
    let s = "Hello World";
    println!("\"{}\" reversed is \"{}\"", s, reverse_words(s));
}