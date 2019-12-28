//* https://www.codewars.com/kata/integers-recreation-one

use std::collections::HashMap;

pub type Divisors = Vec<u64>;
pub type Primes = HashMap<u64, u64>;

pub fn list_squared(m: u64, n: u64) -> Vec<(u64, u64)> {
    // Create vec with maximum possible capacity for exclude memory allocation
    let size = (n as f64).sqrt() as usize;
    let mut divisors = Divisors::with_capacity(size);
    let mut primes = Primes::with_capacity(size);

    (m..=n)
        .filter_map(|num| {
            get_divisors(num, &mut primes, &mut divisors);
            let sum = divisors.iter().map(|num| num * num).sum::<u64>() as f64;
            if sum.sqrt().fract() == 0.0 {
                Some((num, sum as u64))
            } else {
                None
            }
        })
        .collect::<Vec<(u64, u64)>>()
}

pub fn get_divisors(num: u64, primes: &mut Primes, divisors: &mut Divisors) {
    primes.clear();
    divisors.clear();
    // look for primes
    trial_prime(num, primes);
    // copy them to divisors vec
    primes.iter().for_each(|(prime, _)| divisors.push(*prime));
    // get all other divisors
    generate_divisors(0, 1, primes, divisors);
}

// Optimize trial algorithm
pub fn trial_prime(n: u64, primes: &mut Primes) {
    primes.clear();
    let mut num = n;
    while num % 2 == 0 {
        let counter = primes.entry(2).or_insert(0);
        *counter += 1;
        num /= 2;
    }
    let mut divider = 3u64;
    while divider.pow(2) <= num {
        if num % divider == 0 {
            let counter = primes.entry(divider).or_insert(0);
            *counter += 1;
            num /= divider;
        } else {
            divider += 2;
        }
    }
    if num != 1 {
        let counter = primes.entry(num).or_insert(0);
        *counter += 1;
    }
}

pub fn generate_divisors(index: usize, divisor: u64, primes: &mut Primes, divisors: &mut Divisors) {
    let mut divisor = divisor;
    if index == primes.len() {
        if !divisors.contains(&divisor) {
            divisors.push(divisor);
        }
        return;
    }
    // use divisors vec as index
    let prime = divisors.get(index).unwrap();
    let count = primes.get(&prime).unwrap();
    let prime = prime.clone();
    for _ in 0..=*count {
        generate_divisors(index + 1, divisor, primes, divisors);
        divisor *= prime;
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    fn testing(m: u64, n: u64, exp: Vec<(u64, u64)>) -> () {
        assert_eq!(list_squared(m, n), exp)
    }
    #[test]
    fn basics_list_squared() {
        testing(1, 250, vec![(1, 1), (42, 2500), (246, 84100)]);
        testing(1, 250, vec![(1, 1), (42, 2500), (246, 84100)]);
        testing(42, 250, vec![(42, 2500), (246, 84100)]);
        testing(300, 600, vec![]);
    }
}
//* I use this solution in codewars, because it's faster on this site.
//* But criterion calculate, what this solution is 2 time's slower than above.
// Best execution time: ~4.25s
fn list_squared_codewars(m: u64, n: u64) -> Vec<(u64, u64)> {
    // Create vec with maximum possible capacity for exclude memory allocation
    let size = (n as f64).sqrt() as u64;
    let mut divisors = Vec::<u64>::with_capacity(size as usize);

    (m..=n)
        .filter_map(|num| {
            divisors.clear();
            find_divisors(num, &mut divisors);
            let sum = divisors.iter().map(|num| num * num).sum::<u64>() as f64;
            if sum.sqrt().fract() == 0.0 {
                Some((num, sum as u64))
            } else {
                None
            }
        })
        .collect::<Vec<(u64, u64)>>()
}
fn find_divisors(num: u64, divisors: &mut Vec<u64>) {
    let num_sqrt = (num as f64).sqrt() as u64;
    for i in 1..=num_sqrt {
        if num % i == 0 {
            divisors.push(i);
            if i.pow(2) != num {
                divisors.push(num / i);
            }
        }
    }
}