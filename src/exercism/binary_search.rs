use std::cmp::Ordering;

// optimized
pub fn find<A, K>(array: A, key: K) -> Option<usize>
where
    K: Ord,
    A: AsRef<[K]>,
{
    let array = array.as_ref();
    if array.len() == 0 {
        return None;
    }
    let idx = array.len() / 2;
    match key.cmp(&array[idx]) {
        Ordering::Equal => Some(idx),
        Ordering::Less => find(&array[..idx], key),
        Ordering::Greater => find(&array[idx + 1..], key).map(|step| idx + step + 1),
    }
}

// impl by myself
pub fn find1<A, K>(array: A, key: K) -> Option<usize>
where
    K: Ord,
    A: AsRef<[K]>,
{
    let array = array.as_ref();
    match (array.len(), array.len() / 2) {
        (len, _) if len == 0 => None,
        (_, idx) if array[idx] == key => Some(idx),
        (_, idx) if idx == 0 => None,
        (_, idx) if array[idx] > key => find(&array[..idx], key),
        (_, idx) if array[idx] < key => find(&array[idx + 1..], key).map(|step| idx + step + 1),
        _ => None,
    }
}
