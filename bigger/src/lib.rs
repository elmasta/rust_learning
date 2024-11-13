use std::collections::HashMap;

pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let mut max_value = std::i32::MIN;
    for &value in h.values() {
        if value > max_value {
            max_value = value;
        }
    }
    return max_value
}
