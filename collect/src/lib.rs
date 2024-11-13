pub fn bubble_sort(vec: &mut Vec<i32>) {
    for i in 0..vec.len() {
        let mut swapped = false;
        for j in 0..(vec.len() - i - 1) {
            if vec[j] > vec[j + 1] {
                vec.swap(j, j + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

}
