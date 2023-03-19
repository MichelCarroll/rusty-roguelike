use std::cmp;

use js_sys::Math::random;



pub fn random_in_range(x: u64, y: u64) -> u64 {
    let range = y - x + 1;
    let random_ratio = random();
    cmp::min(x + ((range as f64) * random_ratio) as u64, y)
}



pub fn random_in_vec<'a, T>(vector: &'a Vec<T>) -> Option<&'a T> {
    if vector.is_empty() {
        return None 
    }
    let len = vector.len();
    let i = random_in_range(0, (len - 1) as u64);
    vector.get(i as usize) 
}

