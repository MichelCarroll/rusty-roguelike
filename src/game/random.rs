use std::cmp;

use js_sys::Math::random;

pub fn random_in_range(x: u64, y: u64) -> u64 {
    let range = y - x + 1;
    let random_ratio = random();
    cmp::min(x + ((range as f64) * random_ratio) as u64, y)
}

pub fn random_index_vec<'a, T>(vector: &'a Vec<T>) -> Option<usize> {
    if vector.is_empty() {
        return None 
    }
    let len = vector.len();
    let i = random_in_range(0, (len - 1) as u64);
    Some(i as usize)
}

pub fn random_in_vec<'a, T>(vector: &'a Vec<T>) -> Option<&'a T> {
    let index = random_index_vec(vector);
    index.and_then(|i| vector.get(i))
}

pub fn random_in_vec_and_remove<'a, T>(vector: &'a mut Vec<T>) -> Option<T> {
    let index = random_index_vec(vector);
    index.and_then(|i| vector.remove(i).into())
}