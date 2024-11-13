pub fn spell(n: u32) -> String {
    if n == 0 {
        return String::from("zero");
    }
    let ones = [
        "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", 
        "seventeen", "eighteen", "nineteen",
    ];
    let tens = [
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];
    let thousands = ["", "thousand", "million"];
    
    fn spell_chunk(chunk: u32, ones: &[&str], tens: &[&str]) -> String {
        let mut result = String::new();
        let hundreds = chunk / 100;
        let remainder = chunk % 100;
        let ten = remainder / 10;
        let unit = remainder % 10;

        if hundreds > 0 {
            result.push_str(ones[hundreds as usize]);
            result.push_str(" hundred ");
        }

        if remainder > 0 {
            if remainder < 20 {
                result.push_str(ones[remainder as usize]);
            } else {
                result.push_str(tens[ten as usize]);
                if unit > 0 {
                    result.push_str("-");
                    result.push_str(ones[unit as usize]);
                }
            }
        }

        result.trim().to_string()
    }

    let mut num = n;
    let mut result = String::new();
    let mut thousand_index = 0;

    while num > 0 {
        let chunk = num % 1000;
        if chunk > 0 {
            let chunk_spelled = spell_chunk(chunk, &ones, &tens);
            if thousand_index > 0 {
                result = format!("{} {} {}", chunk_spelled, thousands[thousand_index], result);
            } else {
                result = format!("{} {}", chunk_spelled, result);
            }
        }
        num /= 1000;
        thousand_index += 1;
    }

    result.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(spell(0), String::from("zero"));
        assert_eq!(spell(1), String::from("one"));
        assert_eq!(spell(14), String::from("fourteen"));
        assert_eq!(spell(20), String::from("twenty"));
        assert_eq!(spell(22), String::from("twenty-two"));
        assert_eq!(spell(101), String::from("one hundred one"));
        assert_eq!(spell(120), String::from("one hundred twenty"));
        assert_eq!(spell(123), String::from("one hundred twenty-three"));
        assert_eq!(spell(1000), String::from("one thousand"));
        assert_eq!(spell(1055), String::from("one thousand fifty-five"));
        assert_eq!(
            spell(1234),
            String::from("one thousand two hundred thirty-four")
        );
        assert_eq!(
            spell(10123),
            String::from("ten thousand one hundred twenty-three")
        );
        assert_eq!(
            spell(910112),
            String::from("nine hundred ten thousand one hundred twelve")
        );
        assert_eq!(
            spell(651123),
            String::from("six hundred fifty-one thousand one hundred twenty-three")
        );

        assert_eq!(spell(810000), String::from("eight hundred ten thousand"));
        assert_eq!(spell(1000000), String::from("one million"));
    }
}
