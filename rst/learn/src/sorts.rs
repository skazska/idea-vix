/**
 * Quick sort implementation.
 * Choose a Pivot: Select an element from the array as the pivot. The choice of pivot can vary (e.g., first element, last element, random element, or median).
 * Partition the Array: Rearrange the array around the pivot. After partitioning, all elements smaller than the pivot will be on its left, and all elements greater than the pivot will be on its right. The pivot is then in its correct position, and we obtain the index of the pivot.
 * Recursively Call: Recursively apply the same process to the two partitioned sub-arrays (left and right of the pivot).
 * Base Case: The recursion stops when there is only one element left in the sub-array, as a single element is already sorted.
 */

/**
 * choose a pivot
 * bypassing slice in 2 directions, swaping elements if right is less than left, choose a pivot meeting point.
 * retrns the None if no further sort and split is needed (values length is 1 or 0)
 * returns Some(pivot_index) otherwise.
 */ 
fn get_pivot<V>(values: &mut [V]) -> Option<usize>
where V: PartialOrd + Clone + Copy + std::fmt::Debug {
    let right = values.len() - 1;
    let mut r = right;

    for l in 0..=right {
        while r > l && values[r] >= values[l] { r -= 1; }

        if r <= l { 
            return if right > 1 { Some(l) } else { None }; 
        }

        println!("Swapping values at positions {} and {}: {:?} <-> {:?}", l, r, values[l], values[r]);
        values.swap(l, r);
        println!("Values after swap: {:?}", values);
    };

    None
}

// splits slice at pivot position, calls sort_quick recursively on both parts
fn split_and_sort<V>(values: &mut [V], pivot_pos: usize) 
where V: PartialOrd + Clone + Copy + std::fmt::Debug {
    println!("Splitting values: {:?}, pivot_pos: {}", values, pivot_pos);

    let slices = values.split_at_mut(pivot_pos + 1);

    println!("Left part: {:?}, Right part: {:?}", slices.0, slices.1);
    
    // sort the left part
    sort_quick(slices.0);
    // sort the right part, excluding the pivot
    sort_quick(slices.1);
}

// Quick sort implementation
pub fn sort_quick<V>(values: &mut [V]) 
where V: PartialOrd + Clone + Copy + std::fmt::Debug {
    let pivot = get_pivot(values);

    if let Some(pivot_pos) = pivot {
        println!("Pivot found at position: {}", pivot_pos);
        
        split_and_sort(values, pivot_pos);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_get_pivot_tl(vec: &mut [i32], expected_pivot: Option<usize>, expected_values: &[i32]) {
        let pivot = get_pivot(vec);

        assert_eq!(vec, expected_values, "Expected sorted values to be {:?}, but got {:?}", expected_values, vec);

        if vec.len() < 2 {
            assert!(pivot.is_none(), "Expected no pivot for a vector with length < 2, but got {:?}", pivot);
            return;
        }

        match expected_pivot {
            Some(index) => { 
                assert!(pivot.is_some(), "Expected a pivot to be found, but got None");
                let pivot_index = pivot.unwrap();
                assert_eq!(pivot_index, index, "Expected pivot index to be {}, but got {}", index, pivot_index);
            },
            None => assert!(pivot.is_none(), "Expected no pivot, but got {:?}", pivot),
            
        }
    }

    #[test]
    fn test_get_pivot() {
        test_get_pivot_tl(&mut vec![1], None, &vec![1]);
        test_get_pivot_tl(&mut vec![1, 2], None, &vec![1, 2]);
        test_get_pivot_tl(&mut vec![2, 1], None, &vec![1, 2]);
        test_get_pivot_tl(&mut vec![3, 2, 1], Some(1), &vec![1, 2, 3]);
        test_get_pivot_tl(&mut vec![1, 2, 3], Some(0), &vec![1, 2, 3]);
        test_get_pivot_tl(&mut vec![5, 3, 1, 4, 2], Some(2), &vec![2, 1, 3, 4, 5]);
    }

    fn test_sort_quick_tl(vec: &mut [i32], expected_values: &[i32]) {
        sort_quick(vec);
        assert_eq!(vec, expected_values, "Expected sorted values to be {:?}, but got {:?}", expected_values, vec);
    }

    // fn get_random_vec(len: usize) -> Vec<i32> {
    //     vec![rand::rng().random_range(1..=100); len]
    // }

    #[test]
    fn test_sort_quick() {
        // test_sort_quick_tl(&mut vec![1], &vec![1]);
        // test_sort_quick_tl(&mut vec![1, 2], &vec![1, 2]);
        // test_sort_quick_tl(&mut vec![2, 1], &vec![1, 2]);
        // test_sort_quick_tl(&mut vec![3, 2, 1], &vec![1, 2, 3]);
        // test_sort_quick_tl(&mut vec![1, 2, 3], &vec![1, 2, 3]);
        test_sort_quick_tl(&mut vec![5, 3, 1, 4, 2], &vec![1, 2, 3, 4, 5]);
        test_sort_quick_tl(&mut vec![3, 3, 9, 4, 2, 6], &vec![2, 3, 3, 4, 6, 9]);
    }
}