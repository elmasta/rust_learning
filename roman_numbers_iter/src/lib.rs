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

impl Iterator for RomanNumber {
    type Item = RomanNumber;

    fn next(&mut self) -> Option<RomanNumber> {
        if self.0.len() == 0 {
            Some(RomanNumber::from(1))
        } else {
            let mut numerical_nmb: i32 = 0;
            let mut skip = false;
            for (index, roman_nmb) in &mut self.0.iter().enumerate() {
                if skip {
                    skip = false;
                } else {
                    match roman_nmb {
                        I => {
                            if index+1 != self.0.len() {
                                if self.0[index+1] == V {
                                    numerical_nmb += 4;
                                    skip = true
                                } else if self.0[index+1] == X {
                                    numerical_nmb += 9;
                                    skip = true
                                } else {
                                    numerical_nmb += 1
                                }
                            } else {
                                numerical_nmb += 1
                            }},
                        V => numerical_nmb += 5,
                        X => {
                            if index+1 != self.0.len() {
                                if self.0[index+1] == L {
                                    numerical_nmb += 40;
                                    skip = true
                                } else if self.0[index+1] == C {
                                    numerical_nmb += 90;
                                    skip = true
                                } else {
                                    numerical_nmb += 10
                                }
                            } else {
                                numerical_nmb += 10
                            }},
                        L => numerical_nmb += 50,
                        C => {
                            if index+1 != self.0.len() {
                                if self.0[index+1] == D {
                                    numerical_nmb += 400;
                                    skip = true
                                } else if self.0[index+1] == M {
                                    numerical_nmb += 900;
                                    skip = true
                                } else {
                                    numerical_nmb += 100
                                }
                            } else {
                                numerical_nmb += 100
                            }},
                        D => numerical_nmb += 500,
                        M => numerical_nmb += 1000,
                        _ => {},
                    }
                }
            }
            println!("{:?}", numerical_nmb);
            numerical_nmb += 1;
            Some(RomanNumber::from(numerical_nmb as u32))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roman_numbers_iterator_test() {
        // println!("{:?}", RomanNumber::from(9));
        // println!("{:?}", RomanNumber::from(9).next().unwrap());
        // assert_eq!(
        //     1,
        //     0
        // );
        assert_eq!(
            RomanNumber::from(9).0,
            RomanNumber::from(8).next().unwrap().0
        );
        assert_eq!(
            RomanNumber::from(6).0,
            RomanNumber::from(5).next().unwrap().0
        );
        assert_eq!(
            RomanNumber::from(14).0,
            RomanNumber::from(13).next().unwrap().0
        );
        assert_eq!(
            RomanNumber::from(34).0,
            RomanNumber::from(33).next().unwrap().0
        );
        assert_eq!(
            RomanNumber::from(50).0,
            RomanNumber::from(49).next().unwrap().0
        );
        assert_eq!(
            RomanNumber::from(200).0,
            RomanNumber::from(199).next().unwrap().0
        );
        assert_eq!(
            RomanNumber::from(500).0,
            RomanNumber::from(499).next().unwrap().0
        );
        assert_eq!(
            RomanNumber::from(1533).0,
            RomanNumber::from(1532).next().unwrap().0
        );
        assert_eq!(
            RomanNumber::from(2349).0,
            RomanNumber::from(2348).next().unwrap().0
        );
    }
}
