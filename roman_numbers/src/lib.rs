use crate::RomanDigit::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    IV,
    V,
    IX,
    X,
    XL,
    L,
    XC,
    C,
    CD,
    D,
    CM,
    M,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanDigit {
    fn from(value: u32) -> Self {
        match value {
            1 => I,
            4 => IV,
            5 => V,
            9 => IX,
            10 => X,
            40 => XL,
            50 => L,
            90 => XC,
            100 => C,
            400 => CD,
            500 => D,
            900 => CM,
            1000 => M,
            _ => Nulla,
        }
    }
}

impl From<u32> for RomanNumber {
    fn from(mut value: u32) -> Self {
        let digits = [
            (1000, M),
            (900, CM),
            (500, D),
            (400, CD),
            (100, C),
            (90, XC),
            (50, L),
            (40, XL),
            (10, X),
            (9, IX),
            (5, V),
            (4, IV),
            (1, I),
        ];

        let mut result = Vec::new();
        
        for &(num, digit) in &digits {
            while value >= num {
                value -= num;
                match digit {
                    CM => {result.push(C); result.push(M);},
                    CD => {result.push(C); result.push(D);},
                    XC => {result.push(X); result.push(C);},
                    XL => {result.push(X); result.push(L);},
                    IX => {result.push(I); result.push(X);},
                    IV => {result.push(I); result.push(V);},
                    _ => {result.push(digit);},
                }
            }
        }

        RomanNumber(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    //use roman_numbers::RomanDigit::*;
    #[allow(unused_imports)]
    //use roman_numbers::RomanNumber;

    #[test]
    fn it_works() {
        assert_eq!(RomanNumber::from(3).0, [I, I, I]);
        assert_eq!(RomanNumber::from(6).0, [V, I]);
        assert_eq!(RomanNumber::from(15).0, [X, V]);
        assert_eq!(RomanNumber::from(30).0, [X, X, X]);
        assert_eq!(RomanNumber::from(150).0, [C, L]);
        assert_eq!(RomanNumber::from(200).0, [C, C]);
        assert_eq!(RomanNumber::from(600).0, [D, C]);
        assert_eq!(RomanNumber::from(1500).0, [M, D]);
    }

    #[test]
    fn substractive_notation() {
        assert_eq!(RomanNumber::from(4).0, [I, V]);
        assert_eq!(RomanNumber::from(44).0, [X, L, I, V]);
        assert_eq!(RomanNumber::from(3446).0, [M, M, M, C, D, X, L, V, I]);
        assert_eq!(RomanNumber::from(9).0, [I, X]);
        assert_eq!(RomanNumber::from(94).0, [X, C, I, V]);
    }
}
