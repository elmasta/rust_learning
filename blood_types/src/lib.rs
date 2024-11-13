use std::fmt::{self, Debug};
use std::cmp::{Ord, Ordering};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
	A,
	AB,
	B,
	O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
	Positive,
	Negative,
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
	pub antigen: Antigen,
	pub rh_factor: RhFactor,
}

impl FromStr for Antigen {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Antigen::A),
            "AB" => Ok(Antigen::AB),
            "B" => Ok(Antigen::B),
            "O" => Ok(Antigen::O),
            _ => Err(()),
        }
    }
}

impl FromStr for RhFactor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(RhFactor::Positive),
            "-" => Ok(RhFactor::Negative),
            _ => Err(()),
        }
    }
}

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.antigen == other.antigen {
            self.rh_factor.cmp(&other.rh_factor)
        } else {
            self.antigen.cmp(&other.antigen)
        }
    }
}

impl FromStr for BloodType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {
            return Err(());
        }
        let antigen_str = &s[..s.len() - 1];
        let rh_factor_str = &s[s.len() - 1..];
        let antigen = match Antigen::from_str(antigen_str) {
            Ok(antigen) => antigen,
            Err(_) => return Err(()),
        };
        let rh_factor = match RhFactor::from_str(rh_factor_str) {
            Ok(rh_factor) => rh_factor,
            Err(_) => return Err(()),
        };

        Ok(BloodType { antigen, rh_factor })
    }
}

impl Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}",
            match self.antigen {
                Antigen::A => "A",
                Antigen::AB => "AB",
                Antigen::B => "B",
                Antigen::O => "O",
            },
            match self.rh_factor {
                RhFactor::Positive => "+",
                RhFactor::Negative => "-",
            }
        )
    }
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        if self.rh_factor == RhFactor::Negative {
            match self.antigen.clone() {
                Antigen::AB => other.rh_factor == RhFactor::Negative,
                Antigen::A => other.rh_factor == RhFactor::Negative && (other.antigen == Antigen::O || other.antigen == Antigen::A),
                Antigen::B => other.rh_factor == RhFactor::Negative && (other.antigen == Antigen::O || other.antigen == Antigen::B),
                Antigen::O => other.rh_factor == RhFactor::Negative && other.antigen == Antigen::O,
            }
        } else {
            match self.antigen.clone() {
                Antigen::AB => true,
                Antigen::A => other.antigen == Antigen::O || other.antigen == Antigen::A,
                Antigen::B => other.antigen == Antigen::O || other.antigen == Antigen::B,
                Antigen::O => other.antigen == Antigen::O,
            }
        }
    }

    pub fn donors(&self) -> Vec<BloodType> {
        let mut donors = vec![];
        if self.rh_factor == RhFactor::Negative {
            match self.antigen {
                Antigen::A => {
                    donors.push(BloodType {antigen: Antigen::A, rh_factor: RhFactor::Negative});
                    donors.push(BloodType {antigen: Antigen::O, rh_factor: RhFactor::Negative});
                },
                Antigen::B => {
                    donors.push(BloodType {antigen: Antigen::B, rh_factor: RhFactor::Negative});
                    donors.push(BloodType {antigen: Antigen::O, rh_factor: RhFactor::Negative});
                },
                Antigen::AB => {
                    donors.push(BloodType {antigen: Antigen::O, rh_factor: RhFactor::Negative});
                    donors.push(BloodType {antigen: Antigen::A, rh_factor: RhFactor::Negative});
                    donors.push(BloodType {antigen: Antigen::B, rh_factor: RhFactor::Negative});
                    donors.push(BloodType {antigen: Antigen::AB, rh_factor: RhFactor::Negative});
                },
                Antigen::O => {
                    donors.push(BloodType {antigen: Antigen::O, rh_factor: RhFactor::Negative});
                }
            }
        } else {
            match self.antigen {
                Antigen::A => {
                    donors.push(BloodType {antigen: Antigen::A, rh_factor: RhFactor::Negative});
                    donors.push(BloodType {antigen: Antigen::A, rh_factor: RhFactor::Positive});
                    donors.push(BloodType {antigen: Antigen::O, rh_factor: RhFactor::Negative});
                    donors.push(BloodType {antigen: Antigen::O, rh_factor: RhFactor::Positive});
                },
                Antigen::B => {
                    donors.push(BloodType {antigen: Antigen::B, rh_factor: RhFactor::Negative});
                    donors.push(BloodType {antigen: Antigen::B, rh_factor: RhFactor::Positive});
                    donors.push(BloodType {antigen: Antigen::O, rh_factor: RhFactor::Negative});
                    donors.push(BloodType {antigen: Antigen::O, rh_factor: RhFactor::Positive});
                },
                Antigen::AB => {
                    donors.push(BloodType {antigen: Antigen::O, rh_factor: RhFactor::Negative});
                    donors.push(BloodType {antigen: Antigen::O, rh_factor: RhFactor::Positive});
                    donors.push(BloodType {antigen: Antigen::A, rh_factor: RhFactor::Negative});
                    donors.push(BloodType {antigen: Antigen::A, rh_factor: RhFactor::Positive});
                    donors.push(BloodType {antigen: Antigen::B, rh_factor: RhFactor::Negative});
                    donors.push(BloodType {antigen: Antigen::B, rh_factor: RhFactor::Positive});
                    donors.push(BloodType {antigen: Antigen::AB, rh_factor: RhFactor::Negative});
                    donors.push(BloodType {antigen: Antigen::AB, rh_factor: RhFactor::Positive});
                },
                Antigen::O => {
                    donors.push(BloodType {antigen: Antigen::O, rh_factor: RhFactor::Negative});
                    donors.push(BloodType {antigen: Antigen::O, rh_factor: RhFactor::Positive});
                }
            }
        }
        donors
    }

    pub fn recipients(&self) -> Vec<BloodType> {
        let mut recipients = vec![];
            if self.rh_factor == RhFactor::Negative {
                match self.antigen {
                    Antigen::A => {
                        recipients.push(BloodType {antigen: Antigen::A, rh_factor: RhFactor::Negative});
                        recipients.push(BloodType {antigen: Antigen::A, rh_factor: RhFactor::Positive});
                        recipients.push(BloodType {antigen: Antigen::AB, rh_factor: RhFactor::Negative});
                        recipients.push(BloodType {antigen: Antigen::AB, rh_factor: RhFactor::Positive});
                    },
                    Antigen::B => {
                        recipients.push(BloodType {antigen: Antigen::B, rh_factor: RhFactor::Negative});
                        recipients.push(BloodType {antigen: Antigen::B, rh_factor: RhFactor::Positive});
                        recipients.push(BloodType {antigen: Antigen::AB, rh_factor: RhFactor::Negative});
                        recipients.push(BloodType {antigen: Antigen::AB, rh_factor: RhFactor::Positive});
                    },
                    Antigen::AB => {
                        recipients.push(BloodType {antigen: Antigen::AB, rh_factor: RhFactor::Negative});
                        recipients.push(BloodType {antigen: Antigen::AB, rh_factor: RhFactor::Positive});
                    },
                    Antigen::O => {
                        recipients.push(BloodType {antigen: Antigen::O, rh_factor: RhFactor::Negative});
                        recipients.push(BloodType {antigen: Antigen::O, rh_factor: RhFactor::Positive});
                        recipients.push(BloodType {antigen: Antigen::A, rh_factor: RhFactor::Negative});
                        recipients.push(BloodType {antigen: Antigen::A, rh_factor: RhFactor::Positive});
                        recipients.push(BloodType {antigen: Antigen::B, rh_factor: RhFactor::Negative});
                        recipients.push(BloodType {antigen: Antigen::B, rh_factor: RhFactor::Positive});
                        recipients.push(BloodType {antigen: Antigen::AB, rh_factor: RhFactor::Negative});
                        recipients.push(BloodType {antigen: Antigen::AB, rh_factor: RhFactor::Positive});
                    }
                }
            } else {
                match self.antigen {
                    Antigen::A => {
                        recipients.push(BloodType {antigen: Antigen::A, rh_factor: RhFactor::Positive});
                        recipients.push(BloodType {antigen: Antigen::AB, rh_factor: RhFactor::Positive});
                    }
                    Antigen::B => {
                        recipients.push(BloodType {antigen: Antigen::B, rh_factor: RhFactor::Positive});
                        recipients.push(BloodType {antigen: Antigen::AB, rh_factor: RhFactor::Positive});
                    }
                    Antigen::AB => {
                        recipients.push(BloodType {antigen: Antigen::AB, rh_factor: RhFactor::Positive});
                    }
                    Antigen::O => {
                        recipients.push(BloodType {antigen: Antigen::O, rh_factor: RhFactor::Positive});
                        recipients.push(BloodType {antigen: Antigen::A, rh_factor: RhFactor::Positive});
                        recipients.push(BloodType {antigen: Antigen::B, rh_factor: RhFactor::Positive});
                        recipients.push(BloodType {antigen: Antigen::AB, rh_factor: RhFactor::Positive});
                    }
                }
            }
        recipients
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compatible_ab_neg_with_a_pos() {
        let blood_type: BloodType = "AB-".parse().unwrap();
        let other_bt: BloodType = "A+".parse().unwrap();
        println!("{:?}", !blood_type.can_receive_from(&other_bt));
        assert!(!blood_type.can_receive_from(&other_bt));
    }

    #[test]
    fn compatible_a_neg_with_a_pos() {
        let blood_type: BloodType = "A-".parse().unwrap();
        let other_bt: BloodType = "A+".parse().unwrap();
        assert!(!blood_type.can_receive_from(&other_bt));
    }

    #[test]
    fn compatible_a_neg_with_ab_neg() {
        let blood_type: BloodType = "AB-".parse().unwrap();
        let other_bt: BloodType = "A-".parse().unwrap();
        assert!(blood_type.can_receive_from(&other_bt));
    }

    #[test]
    fn compatible_ab_neg_with_o_pos() {
        let blood_type: BloodType = "AB-".parse().unwrap();
        let other_bt: BloodType = "O+".parse().unwrap();
        println!("{:?}", !blood_type.can_receive_from(&other_bt));
        assert!(!blood_type.can_receive_from(&other_bt));
    }

    #[test]
    fn compatible_ab_pos_with_o_pos() {
        let blood_type: BloodType = "AB+".parse().unwrap();
        let other_bt: BloodType = "O+".parse().unwrap();
        assert!(blood_type.can_receive_from(&other_bt));
    }

    #[test]
    fn test_compatible_ab_neg_with_o_neg() {
        let blood_type: BloodType = "AB-".parse().unwrap();
        let other_bt: BloodType = "O-".parse().unwrap();
        assert!(blood_type.can_receive_from(&other_bt));
    }

    #[test]
    fn test_antigen_ab_from_str() {
        let blood = "AB+";
        let blood_type: BloodType = blood.parse().unwrap();
        assert_eq!(blood_type.antigen, Antigen::AB);
        assert_eq!(blood_type.rh_factor, RhFactor::Positive);
    }

    #[test]
    fn test_antigen_a_from_str() {
        let blood = "A-";
        let blood_type = blood.parse::<BloodType>().unwrap();
        assert_eq!(blood_type.antigen, Antigen::A);
        assert_eq!(blood_type.rh_factor, RhFactor::Negative);
    }

    #[test]
    #[should_panic]
    fn test_unexistent_blood_type() {
        let _blood_type: BloodType = "AO-".parse().unwrap();
    }

    #[test]
    fn test_donors() {
        let mut givers = "AB+".parse::<BloodType>().unwrap().donors();
        println!("Before sorting {:?}", &givers);
        givers.sort();
        println!("{:?}", &givers);
        let mut expected = vec![
            "AB-".parse::<BloodType>().unwrap(),
            "A-".parse().unwrap(),
            "B-".parse().unwrap(),
            "O-".parse().unwrap(),
            "AB+".parse().unwrap(),
            "A+".parse().unwrap(),
            "B+".parse().unwrap(),
            "O+".parse().unwrap(),
        ];
        expected.sort();
        assert_eq!(givers, expected);
    }

    #[test]
    fn test_a_neg_donors() {
        let mut givers = "A-".parse::<BloodType>().unwrap().donors();
        givers.sort();
        let mut expected: Vec<BloodType> = vec!["A-".parse().unwrap(), "O-".parse().unwrap()];
        expected.sort();
        assert_eq!(givers, expected);
    }

    #[test]
    fn test_o_neg_donors() {
        let mut givers = "O-".parse::<BloodType>().unwrap().donors();
        givers.sort();
        let mut expected: Vec<BloodType> = vec!["O-".parse().unwrap()];
        expected.sort();
        assert_eq!(givers, expected);
    }

    #[test]
    fn test_ab_pos_recipients() {
        let mut recipients: Vec<BloodType> = "AB+".parse::<BloodType>().unwrap().recipients();
        recipients.sort();
        let mut expected: Vec<BloodType> = vec!["AB+".parse().unwrap()];
        expected.sort();
        println!("{:?} {:?}", recipients, expected);
        assert_eq!(recipients, expected);
    }

    #[test]
    fn test_a_neg_recipients() {
        let mut recipients = "A-".parse::<BloodType>().unwrap().recipients();
        recipients.sort();
        let mut expected: Vec<BloodType> = vec![
            "A-".parse().unwrap(),
            "AB+".parse().unwrap(),
            "A+".parse().unwrap(),
            "AB-".parse().unwrap(),
        ];
        expected.sort();
        assert_eq!(recipients, expected);
    }

    #[test]
    fn test_output() {
        let blood_type: BloodType = "O+".parse().unwrap();
        println!("recipients of O+ {:?}", blood_type.recipients());
        println!("donors of O+ {:?}", blood_type.donors());
        let another_blood_type: BloodType = "A-".parse().unwrap();
        println!(
            "donors of O+ can receive from {:?} {:?}",
            &another_blood_type,
            blood_type.can_receive_from(&another_blood_type)
        );
    }
    }
