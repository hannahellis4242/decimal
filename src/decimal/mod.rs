#[derive(Debug)]
enum Symbol {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Symbol::Zero => match other {
                Symbol::Zero => true,
                _ => false,
            },
            Symbol::One => match other {
                Symbol::One => true,
                _ => false,
            },
            Symbol::Two => match other {
                Symbol::Two => true,
                _ => false,
            },
            Symbol::Three => match other {
                Symbol::Three => true,
                _ => false,
            },
            Symbol::Four => match other {
                Symbol::Four => true,
                _ => false,
            },
            Symbol::Five => match other {
                Symbol::Five => true,
                _ => false,
            },
            Symbol::Six => match other {
                Symbol::Six => true,
                _ => false,
            },
            Symbol::Seven => match other {
                Symbol::Seven => true,
                _ => false,
            },
            Symbol::Eight => match other {
                Symbol::Eight => true,
                _ => false,
            },
            Symbol::Nine => match other {
                Symbol::Nine => true,
                _ => false,
            },
        }
    }
}

impl Clone for Symbol {
    fn clone(&self) -> Self {
        match self {
            Symbol::Zero => Symbol::Zero,
            Symbol::One => Symbol::One,
            Symbol::Two => Symbol::Two,
            Symbol::Three => Symbol::Three,
            Symbol::Four => Symbol::Four,
            Symbol::Five => Symbol::Five,
            Symbol::Six => Symbol::Six,
            Symbol::Seven => Symbol::Seven,
            Symbol::Eight => Symbol::Eight,
            Symbol::Nine => Symbol::Nine,
        }
    }
}

use std::fmt;
impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Symbol::Zero => write!(f, "0"),
            Symbol::One => write!(f, "1"),
            Symbol::Two => write!(f, "2"),
            Symbol::Three => write!(f, "3"),
            Symbol::Four => write!(f, "4"),
            Symbol::Five => write!(f, "5"),
            Symbol::Six => write!(f, "6"),
            Symbol::Seven => write!(f, "7"),
            Symbol::Eight => write!(f, "8"),
            Symbol::Nine => write!(f, "9"),
        }
    }
}

impl Symbol {
    fn from_char(c: &char) -> Option<Symbol> {
        match c {
            '0' => Some(Symbol::Zero),
            '1' => Some(Symbol::One),
            '2' => Some(Symbol::Two),
            '3' => Some(Symbol::Three),
            '4' => Some(Symbol::Four),
            '5' => Some(Symbol::Five),
            '6' => Some(Symbol::Six),
            '7' => Some(Symbol::Seven),
            '8' => Some(Symbol::Eight),
            '9' => Some(Symbol::Nine),
            _ => None,
        }
    }
}

#[derive(Debug)]
enum Sign {
    Plus,
    Minus,
    NoSign,
}

impl Sign {
    fn from_char(c: &char) -> Sign {
        match c {
            '+' => Sign::Plus,
            '-' => Sign::Minus,
            _ => Sign::NoSign,
        }
    }
}

impl PartialEq for Sign {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Sign::Plus => match other {
                Sign::Plus => true,
                _ => false,
            },
            Sign::Minus => match other {
                Sign::Minus => true,
                _ => false,
            },
            Sign::NoSign => match other {
                Sign::NoSign => true,
                _ => false,
            },
        }
    }
}

#[derive(Debug)]
struct Integer {
    sign: Sign,
    symbols: Vec<Symbol>,
}

impl Integer {
    fn new_raw(s: Sign, ss: &[Symbol]) -> Integer {
        Integer {
            sign: s,
            symbols: ss.to_vec(),
        }
    }
}

impl PartialEq for Integer {
    fn eq(&self, other: &Self) -> bool {
        self.sign == other.sign && self.symbols.iter().eq(other.symbols.iter())
    }
}

use std::str::FromStr;

#[derive(Debug)]
enum ParseIntegerError {
    EmptyString,
    NotANumber,
}

impl PartialEq for ParseIntegerError {
    fn eq(&self, other: &Self) -> bool {
        match self {
            ParseIntegerError::EmptyString => match other {
                ParseIntegerError::EmptyString => true,
                _ => false,
            },
            ParseIntegerError::NotANumber => match other {
                ParseIntegerError::NotANumber => true,
                _ => false,
            },
        }
    }
}

impl FromStr for Integer {
    type Err = ParseIntegerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err(ParseIntegerError::EmptyString)
        } else {
            if s.chars().all(|c| c.is_digit(10) || c == '-' || c == '+') {
                let sign = s
                    .chars()
                    .take(1)
                    .map(|c| Sign::from_char(&c))
                    .fold(Sign::NoSign, |_, x| x);
                s.chars()
                    .skip_while(|c| !c.is_digit(10))
                    .map(|c| Symbol::from_char(&c))
                    .collect::<Option<Vec<_>>>()
                    .ok_or(ParseIntegerError::NotANumber)
                    .map(|s| {
                        s.into_iter()
                            .skip_while(|x| *x == Symbol::Zero)
                            .collect::<Vec<_>>()
                    }) //remove leading zeros
                    .and_then(|s| {
                        if s.is_empty() {
                            Ok(Integer::new_raw(Sign::NoSign, &[Symbol::Zero]))
                        } else {
                            match sign {
                                Sign::Plus => Ok(Integer::new_raw(Sign::Plus, &s)),
                                Sign::Minus => Ok(Integer::new_raw(Sign::Minus, &s)),
                                Sign::NoSign => Ok(Integer::new_raw(Sign::Plus, &s)),
                            }
                        }
                    })
            } else {
                Err(ParseIntegerError::NotANumber)
            }
        }
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sign = match self.sign {
            Sign::Minus => "-".to_string(),
            _ => String::new(),
        };
        let s = self
            .symbols
            .iter()
            .fold(sign, |acc, x| format!("{}{}", acc, x));
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_interger_from_str_0() {
        assert_eq!(
            Integer::from_str("0"),
            Ok(Integer::new_raw(Sign::NoSign, &[Symbol::Zero]))
        );
    }
    #[test]
    fn test_interger_from_str_1() {
        assert_eq!(
            Integer::from_str("1"),
            Ok(Integer::new_raw(Sign::Plus, &[Symbol::One]))
        );
    }
    #[test]
    fn test_interger_from_str_2() {
        assert_eq!(
            Integer::from_str("2"),
            Ok(Integer::new_raw(Sign::Plus, &[Symbol::Two]))
        );
    }
    #[test]
    fn test_interger_from_str_3() {
        assert_eq!(
            Integer::from_str("3"),
            Ok(Integer::new_raw(Sign::Plus, &[Symbol::Three]))
        );
    }
    #[test]
    fn test_interger_from_str_4() {
        assert_eq!(
            Integer::from_str("4"),
            Ok(Integer::new_raw(Sign::Plus, &[Symbol::Four]))
        );
    }
    #[test]
    fn test_interger_from_str_5() {
        assert_eq!(
            Integer::from_str("5"),
            Ok(Integer::new_raw(Sign::Plus, &[Symbol::Five]))
        );
    }
    #[test]
    fn test_interger_from_str_6() {
        assert_eq!(
            Integer::from_str("6"),
            Ok(Integer::new_raw(Sign::Plus, &[Symbol::Six]))
        );
    }
    #[test]
    fn test_interger_from_str_7() {
        assert_eq!(
            Integer::from_str("7"),
            Ok(Integer::new_raw(Sign::Plus, &[Symbol::Seven]))
        );
    }
    #[test]
    fn test_interger_from_str_8() {
        assert_eq!(
            Integer::from_str("8"),
            Ok(Integer::new_raw(Sign::Plus, &[Symbol::Eight]))
        );
    }
    #[test]
    fn test_interger_from_str_9() {
        assert_eq!(
            Integer::from_str("9"),
            Ok(Integer::new_raw(Sign::Plus, &[Symbol::Nine]))
        );
    }
    #[test]
    fn test_interger_from_str_10() {
        assert_eq!(
            Integer::from_str("10"),
            Ok(Integer::new_raw(Sign::Plus, &[Symbol::One, Symbol::Zero]))
        );
    }
    #[test]
    fn test_interger_from_str_empty() {
        assert_eq!(Integer::from_str(""), Err(ParseIntegerError::EmptyString));
    }
    #[test]
    fn test_interger_from_str_not_a_number() {
        assert_eq!(
            Integer::from_str("Hello World"),
            Err(ParseIntegerError::NotANumber)
        );
    }
    #[test]
    fn test_interger_from_str_minus_1() {
        assert_eq!(
            Integer::from_str("-1"),
            Ok(Integer::new_raw(Sign::Minus, &[Symbol::One]))
        );
    }
    #[test]
    fn test_interger_from_str_minus_10() {
        assert_eq!(
            Integer::from_str("-10"),
            Ok(Integer::new_raw(Sign::Minus, &[Symbol::One, Symbol::Zero]))
        );
    }
    #[test]
    fn test_interger_from_str_plus_10() {
        assert_eq!(
            Integer::from_str("+10"),
            Ok(Integer::new_raw(Sign::Plus, &[Symbol::One, Symbol::Zero]))
        );
    }
    #[test]
    fn test_interger_from_str_minus_0() {
        assert_eq!(
            Integer::from_str("-0"),
            Ok(Integer::new_raw(Sign::NoSign, &[Symbol::Zero]))
        );
    }
    #[test]
    fn test_interger_from_str_plus_0() {
        assert_eq!(
            Integer::from_str("+0"),
            Ok(Integer::new_raw(Sign::NoSign, &[Symbol::Zero]))
        );
    }
    #[test]
    fn test_interger_from_str_plus_00000() {
        assert_eq!(
            Integer::from_str("+00000"),
            Ok(Integer::new_raw(Sign::NoSign, &[Symbol::Zero]))
        );
    }
    #[test]
    fn test_interger_from_str_dashes_centre() {
        assert_eq!(
            Integer::from_str("125-12"),
            Err(ParseIntegerError::NotANumber)
        );
    }
    #[test]
    fn test_interger_from_str_pluses_centre() {
        assert_eq!(
            Integer::from_str("125+12"),
            Err(ParseIntegerError::NotANumber)
        );
    }

    #[test]
    fn test_display() {
        assert_eq!(
            format!("{}", Integer::from_str("12512").unwrap()),
            "12512".to_string()
        );
    }
    #[test]
    fn test_display_negative() {
        assert_eq!(
            format!("{}", Integer::from_str("-12512").unwrap()),
            "-12512".to_string()
        );
    }
}
