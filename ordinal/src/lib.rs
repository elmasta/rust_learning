pub fn num_to_ordinal(x: u32) -> String {
    if x % 10 == 1 && x % 100 != 11 {
        x.to_string() + "st"
    } else if x % 10 == 2 && x % 100 != 12 {
        x.to_string() + "nd"
    } else if x % 10 == 3 && x % 100 != 13 {
        x.to_string() + "rd"
    } else {
        x.to_string() + "th"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_to_ordinal() {
        assert_eq!(num_to_ordinal(0), "0th");
        assert_eq!(num_to_ordinal(1), "1st");
        assert_eq!(num_to_ordinal(12), "12th");
        assert_eq!(num_to_ordinal(22), "22nd");
        assert_eq!(num_to_ordinal(43), "43rd");
        assert_eq!(num_to_ordinal(67), "67th");
        assert_eq!(num_to_ordinal(1901), "1901st");
    }
}
