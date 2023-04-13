fn primes_up_to(n: u32) -> Vec<u32> {
    let mut res = Vec::new();
    let mut i = 2;
    for i in 1..n {
        if is_prime(i) {
            res.push(i);
        }
    }
    res
}

fn is_prime(n: u32) -> bool {
    let mut i = 2;
    while i < n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

fn main(){
    let primes = primes_up_to(100);
    println!("{:?}", primes);
}