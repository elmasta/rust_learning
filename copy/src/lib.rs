pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let cfloat = f64::from(c);
    let abs_value = cfloat.abs();
    return (c, cfloat.exp(), abs_value.ln());
}

pub fn str_function(a: String) -> (String, String) {
    let mut to_return = String::new();
    let numbers: Vec<u32> = a
        .split_whitespace()
        .map(|s| s.parse().expect("Conversion error"))
        .collect();
    for number in numbers {
        if to_return.len() != 0 {
            to_return += " ";
        }
        let nmbfloat = f64::from(number);
        let strfloat = nmbfloat.exp().to_string();
        to_return += &strfloat;
    }
    return (a, to_return);
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut to_return = Vec::<f64>::new();
    for number in b.clone() {
        let nmbfloat = f64::from(number);
        let abs_value = nmbfloat.abs();
        to_return.push(abs_value.ln());
    }
    return (b, to_return);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = nbr_function(1);
        assert_eq!(result, (1, 2.718281828459045, 0.0));
    }
}
