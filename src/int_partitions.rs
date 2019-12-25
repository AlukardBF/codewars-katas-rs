//* Kata: https://www.codewars.com/kata/55cf3b567fc0e02b0b00000b

use std::convert::TryFrom;

type Partition = Vec<usize>;

pub fn part(n: i64) -> String {
    let n = usize::try_from(n).unwrap();
    // let partitions: Vec<Vec<u8>> = Vec::with_capacity();
    let mut partitions: Vec<Partition> = Vec::new();
    // Add first partition of all '1'
    partitions.push(vec![1; n]);
    // Get all partitions of our number (n)
    while partitions.last().unwrap().first().unwrap() != &n {
        // Clone latest vector
        let mut last_vec: Partition = partitions.last().unwrap().clone();
        // Get index of first minimum element, skip last
        let iter = &mut last_vec.iter();
        let index_of_min = iter
            .take(iter.len() - 1)
            .enumerate()
            .min_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(index, _)| index)
            .unwrap();
        // transfer '1' from latest element to found minimum
        if let Some(x) = last_vec.last_mut() {
            *x = *x - 1;
        }
        if let Some(x) = last_vec.get_mut(index_of_min) {
            *x = *x + 1;
        }
        // expand the sum of all elements after 'index_of_min'
        let sum: usize = last_vec.split_off(index_of_min + 1).iter().sum();
        last_vec.append(&mut vec![1; sum]);
        // Optimize the used space
        last_vec.shrink_to_fit();
        // Add new partition
        partitions.push(last_vec);
    }
    // Calculate products of partitions
    let mut products: Vec<usize> = partitions
        .iter()
        .map(|partition| partition.iter().product::<usize>())
        .collect();
    // Unstable sorting is preferred, because it's faster and sorting in-place
    products.sort_unstable();
    // Remove duplications
    products.dedup();
    println!("products: {:?}", products);
    // Calculate range
    let range = products.last().unwrap() - products.first().unwrap();
    // Create a closure to calculate mean
    let get_mean = |x: &Vec<usize>| x.iter().sum::<usize>() as f64 / x.len() as f64;
    // Calculate mean for products
    let mean = get_mean(&products);
    // Index of center element
    let mid: usize = products.len() / 2;
    // Calculate median
    let median = if products.len() % 2 == 0 {
        get_mean(&vec![products[mid - 1], products[mid]])
    } else {
        products[mid] as f64
    };
    format!(
        "Range: {0} Average: {1:.2} Median: {2:.2}",
        range, mean, median
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testequal(ans: &str, sol: &str) {
        assert!(ans == sol, "Expected \"{}\", got \"{}\".", sol, ans);
    }
    #[test]
    fn returns_expected() {
        testequal(&part(1), "Range: 0 Average: 1.00 Median: 1.00");
        testequal(&part(2), "Range: 1 Average: 1.50 Median: 1.50");
        testequal(&part(3), "Range: 2 Average: 2.00 Median: 2.00");
        testequal(&part(4), "Range: 3 Average: 2.50 Median: 2.50");
        testequal(&part(5), "Range: 5 Average: 3.50 Median: 3.50");
    }
}
