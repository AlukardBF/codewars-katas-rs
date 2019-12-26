//* https://www.codewars.com/kata/integers-recreation-one

// use std::collections::HashMap;
use indexmap::map::IndexMap;

pub type Divisors = Vec<u64>;
pub type Primes = IndexMap<u64, u64>;

struct PrimeNum {
    count: u64,
    prime: u64,
}

// pub fn list_squared(m: u64, n: u64) -> Vec<(u64, u64)> {
//     // Create vec with maximum possible capacity for exclude memory allocation
//     let size = (n as f64).sqrt() as usize;
//     let mut divisors = Divisors::with_capacity(size);
//     let mut primes = Primes::with_capacity(size / 2);

//     (m..=n)
//         .filter_map(|num| {
//             get_divisors(num, &mut primes, &mut divisors);
//             let sum = divisors.iter().map(|num| num * num).sum::<u64>() as f64;
//             if sum.sqrt().fract() == 0.0 {
//                 Some((num, sum as u64))
//             } else {
//                 None
//             }
//         })
//         .collect::<Vec<(u64, u64)>>()
// }

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

pub fn get_divisors(index: usize, divisor: u64, divisors: &mut Primes) {
    let mut divisor = divisor;
    if index == divisors.len() {
        // print!("{}, ", divisor);
        return;
    }
    let (prime, count) = divisors.get_index(index).unwrap();
    let prime = prime.clone();
    for _ in 0..=*count {
        get_divisors(index + 1, divisor, divisors);
        divisor *= prime;
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_divisors() {
    //     let size = (246 as f64).sqrt() as usize;
    //     let mut divisors = Divisors::with_capacity(size);
    //     let mut primes = Primes::with_capacity(size / 2);
    //     get_divisors(728, &mut primes, &mut divisors);
    //     // divisors.sort_unstable();
    //     println!("{:?}", divisors);
    //     assert!(false)
    //     // assert_eq!(vec![1, 2, 3, 6, 41, 82, 123, 246], divisors);
    // }
    #[test]
    fn test_prime() {
        let size = (246 as f64).sqrt() as usize;
        let mut primes = Primes::with_capacity(size / 2);
        trial_prime(42, &mut primes);

        get_divisors(0, 1, &mut primes);
        // divisors.sort_unstable();
        println!("{:?}", primes);
        assert!(false)
        // assert_eq!(vec![1, 2, 3, 6, 41, 82, 123, 246], divisors);
    }

    // fn testing(m: u64, n: u64, exp: Vec<(u64, u64)>) -> () {
    //     assert_eq!(list_squared(m, n), exp)
    // }
    // #[test]
    // fn basics_list_squared() {
    //     testing(1, 250, vec![(1, 1), (42, 2500), (246, 84100)]);
    //     testing(1, 250, vec![(1, 1), (42, 2500), (246, 84100)]);
    //     testing(42, 250, vec![(42, 2500), (246, 84100)]);
    //     testing(300, 600, vec![]);
    // }
}

pub fn trial_prime_old(n: u64, primes: &mut Vec<u64>) {
    primes.clear();
    let mut num = n;
    if num % 2 == 0 {
        primes.push(2);
        num /= 2;
    }
    while num % 2 == 0 {
        num /= 2;
    }
    let mut divider = 3u64;
    while divider.pow(2) <= num {
        if num % divider == 0 {
            primes.push(divider);
            num /= divider;
        } else {
            divider += 2;
        }
    }
    if num != 1 {
        primes.push(num);
    }
}

pub fn get_divisors_old(num: u64, primes: &mut Vec<u64>, divisors: &mut Vec<u64>) {
    divisors.clear();
    // Looking for primes
    trial_prime_old(num, primes);
    let size = primes.len();

    if size > 2 {
    }
    // Move primes to divisors vector
    divisors.append(primes);
    // Append 1 and num
    divisors.push(1);
    if num != 1 {
        divisors.push(num);
    }
    divisors.sort_unstable();
    divisors.dedup();
}