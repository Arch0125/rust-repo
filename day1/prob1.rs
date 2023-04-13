fn fibonacci(n: u32) {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let temp = a;
        a = b;
        b = temp + b;
    }
    println!("Fibonacci number {} is {}", n, a);
}

fn main(){
    fibonacci(10);
}