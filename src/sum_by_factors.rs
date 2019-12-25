//* https://www.codewars.com/kata/sum-by-factors

use std::collections::BTreeSet;

type Primes = BTreeSet<i64>;

pub fn sum_of_divided(l: Vec<i64>) -> Vec<(i64, i64)> {
    let mut primes = Primes::new();
    l.iter().for_each(|&num| trial_division(num, &mut primes));
    primes
        .iter()
        .map(|prime| {
            (
                *prime,
                l.iter().filter(|num| *num % prime == 0).sum::<i64>(),
            )
        })
        .collect::<Vec<(i64, i64)>>()
}

fn trial_division(n: i64, prime: &mut Primes) {
    let mut num = n.abs();
    if num % 2 == 0 {
        prime.insert(2);
        num = num / 2;
    }
    while num % 2 == 0 {
        num = num / 2;
    }
    let mut divider = 3i64;
    while divider.pow(2) <= num {
        if num % divider == 0 {
            prime.insert(divider);
            num = num / divider;
        } else {
            divider = divider + 2;
        }
    }
    if num != 1 {
        prime.insert(num);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(l: Vec<i64>, exp: Vec<(i64, i64)>) -> () {
        assert_eq!(sum_of_divided(l), exp)
    }
    #[test]
    fn basics_sum_of_divided() {
        testing(vec![12, 15], vec![(2, 12), (3, 27), (5, 15)]);
        testing(
            vec![15, 21, 24, 30, 45],
            vec![(2, 54), (3, 135), (5, 90), (7, 21)],
        );
        testing(
            vec![15, 21, 24, 30, -45],
            vec![(2, 54), (3, 45), (5, 0), (7, 21)],
        );

    }
}
