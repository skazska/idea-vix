// Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) 
// mode (the value that occurs most often; a hash map will be helpful here) of the list.

use std::collections::HashMap;
use crate::sorts;

pub fn median<V>(values: &mut Vec<V>) -> Option<f64>
where V: PartialOrd + Clone + Copy + std::ops::Add<Output = V> + Into<f64> + std::fmt::Debug {
    if values.is_empty() {
        None
    } else {
        sorts::sort_quick(values);

        let len = values.len();
        let mid = len / 2;

        if len % 2 == 0 {
            // If even, return the average of the two middle values
            let mid1 = values[mid - 1];
            let mid2 = values[mid];
            Some((mid1 + mid2).into() / 2.0)
        } else {
            // If odd, return the middle value
            Some(values[mid].into())
        }
    }
}

pub fn mode<V>(values: &Vec<V>) -> Option<V>
where V: Clone + std::fmt::Display
{
    let mut occurances: HashMap<String, (usize, usize)> = HashMap::new();

    for (i, v) in values.iter().enumerate() {
        let counter = occurances.entry(v.to_string()).or_insert((0, i));
        counter.0 += 1; // Increment the count
    };

    let max = occurances.iter().max_by_key(|&(_, count)| count.0).map(|(_, (_, i))| i);

    match max {
        Some(&i) => Some(values[i].clone()),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_median_tl(vec: &Vec<f64>, expected: Option<f64>) {
        let result = median(&mut vec.clone());
        assert_eq!(result, expected, "Expected median of {:?} to be {:?}, but got {:?}", vec, expected, result);
    }

    #[test]
    fn test_median() {
        test_median_tl(&vec![1.0, 2.0, 3.0, 4.0, 5.0], Some(3.0));
        test_median_tl(&vec![1.0, 2.0, 3.0, 4.0], Some(2.5));
        test_median_tl(&vec![5.0, 3.0, 1.0, 4.0, 2.0], Some(3.0));
        test_median_tl(&vec![], None);
    }

    fn test_mode_tl(vec: &Vec<i32>, expected: Option<i32>) {
        let result = mode(vec);
        assert_eq!(result, expected, "Expected mode of {:?} to be {:?}, but got {:?}", vec, expected, result);
    }
    #[test]
    fn test_mode() {
        test_mode_tl(&vec![1, 2, 2, 3, 4, 4], Some(4));
        test_mode_tl(&vec![1, 1, 2, 3, 4], Some(1));
        test_mode_tl(&vec![5, 3, 1, 4, 2], Some(2)); // No mode
        test_mode_tl(&vec![1], Some(1)); // Single element
        test_mode_tl(&vec![], None); // Empty vector
    }
}