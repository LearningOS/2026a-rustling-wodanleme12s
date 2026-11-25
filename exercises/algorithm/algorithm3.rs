/// In-place heap sort: build a max-heap bottom-up, then repeatedly swap the
/// root (the maximum) to the end of the unsorted prefix and sift it back down.
/// Chosen because it needs no scratch buffer and only `Ord` + swapping — the
/// signature here has no `Default`/`Clone` bound to lean on.
fn sort<T>(array: &mut [T])
where
    T: Ord,
{
    let n = array.len();
    // Nodes past n/2 are leaves, so starting at n/2 - 1 and sifting each down
    // turns the whole array into a max-heap in O(n).
    for i in (0..n / 2).rev() {
        sift_down(array, i, n);
    }
    // After each pass the tail holds the largest elements in final order, so the
    // heap shrinks by one instead of the array growing.
    for end in (1..n).rev() {
        array.swap(0, end);
        sift_down(array, 0, end);
    }
}

/// Move the out-of-place value at `start` down to where it belongs, assuming the
/// subtrees below it already satisfy the heap property over `array[..heap_len]`.
fn sift_down<T>(array: &mut [T], mut start: usize, heap_len: usize)
where
    T: Ord,
{
    loop {
        let left = 2 * start + 1;
        if left >= heap_len {
            return;
        }
        // Pick the larger child. `left + 1` is only a valid index when the right
        // child exists, hence the explicit bound instead of slicing.
        let right = left + 1;
        let biggest_child = if right < heap_len && array[right] > array[left] {
            right
        } else {
            left
        };
        if array[start] >= array[biggest_child] {
            return;
        }
        array.swap(start, biggest_child);
        start = biggest_child;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
