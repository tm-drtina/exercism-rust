use std::cmp::Ordering;

pub fn find<T: Ord, A: AsRef<[T]>>(array: A, key: T) -> Option<usize> {
    let array = array.as_ref();
    let mut start = 0;
    let mut end = array.len();
    while start != end {
        let mid = start + (end - start) / 2;
        match key.cmp(&array[mid]) {
            Ordering::Less => end = mid,
            Ordering::Equal => return Some(mid),
            Ordering::Greater => start = mid + 1,
        }
    }
    None
}
