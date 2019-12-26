//* For debug purposes
#[allow(unused_imports)]
use codewars::*;
use codewars::integers_recreating_one::*;
fn main() {
    let size = (246 as f64).sqrt() as usize;
    let mut primes = Primes::with_capacity(size / 2);
    trial_prime(42, &mut primes);

    get_divisors(0, 1, &mut primes);
    println!("{:?}", primes);
}
