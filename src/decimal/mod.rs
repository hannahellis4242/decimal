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
pub struct UnsignedInteger {
    symbols: Vec<Symbol>, //stored least significant digit first, ie units,tens,hundreds etc
}

impl UnsignedInteger {
    fn raw(ss: &[Symbol]) -> UnsignedInteger {
        UnsignedInteger {
            symbols: ss.to_vec(),
        }
    }
}

impl PartialEq for UnsignedInteger {
    fn eq(&self, other: &Self) -> bool {
        self.symbols.iter().eq(other.symbols.iter())
    }
}

use std::str::FromStr;

#[derive(Debug)]
pub enum ParseUnsignedIntegerError {
    EmptyString,
    NotANumber,
}

impl PartialEq for ParseUnsignedIntegerError {
    fn eq(&self, other: &Self) -> bool {
        match self {
            ParseUnsignedIntegerError::EmptyString => match other {
                ParseUnsignedIntegerError::EmptyString => true,
                _ => false,
            },
            ParseUnsignedIntegerError::NotANumber => match other {
                ParseUnsignedIntegerError::NotANumber => true,
                _ => false,
            },
        }
    }
}

impl FromStr for UnsignedInteger {
    type Err = ParseUnsignedIntegerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err(ParseUnsignedIntegerError::EmptyString)
        } else {
            if s.chars().all(|c| c.is_digit(10)) {
                let digits = s
                    .chars()
                    .skip_while(|c| *c == '0') //remove leading zeros
                    .collect::<Vec<_>>();
                digits
                    .iter()
                    .rev()
                    .map(|c| Symbol::from_char(&c))
                    .collect::<Option<Vec<_>>>()
                    .ok_or(ParseUnsignedIntegerError::NotANumber)
                    .and_then(|s| {
                        if s.is_empty() {
                            Ok(UnsignedInteger::raw(&[Symbol::Zero]))
                        } else {
                            Ok(UnsignedInteger::raw(&s))
                        }
                    })
            } else {
                Err(ParseUnsignedIntegerError::NotANumber)
            }
        }
    }
}

impl fmt::Display for UnsignedInteger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self
            .symbols
            .iter()
            .rev()
            .fold(String::new(), |acc, x| format!("{}{}", acc, x));
        write!(f, "{}", s)
    }
}

impl Clone for UnsignedInteger {
    fn clone(&self) -> Self {
        UnsignedInteger {
            symbols: self.symbols.clone(),
        }
    }
}

fn half_add_unit(a: &Symbol, b: &Symbol) -> (Symbol, Symbol) {
    match a {
        Symbol::Zero => match b {
            Symbol::Zero => (Symbol::Zero, Symbol::Zero),
            Symbol::One => (Symbol::One, Symbol::Zero),
            Symbol::Two => (Symbol::Two, Symbol::Zero),
            Symbol::Three => (Symbol::Three, Symbol::Zero),
            Symbol::Four => (Symbol::Four, Symbol::Zero),
            Symbol::Five => (Symbol::Five, Symbol::Zero),
            Symbol::Six => (Symbol::Six, Symbol::Zero),
            Symbol::Seven => (Symbol::Seven, Symbol::Zero),
            Symbol::Eight => (Symbol::Eight, Symbol::Zero),
            Symbol::Nine => (Symbol::Nine, Symbol::Zero),
        },
        Symbol::One => match b {
            Symbol::Zero => (Symbol::One, Symbol::Zero),
            Symbol::One => (Symbol::Two, Symbol::Zero),
            Symbol::Two => (Symbol::Three, Symbol::Zero),
            Symbol::Three => (Symbol::Four, Symbol::Zero),
            Symbol::Four => (Symbol::Five, Symbol::Zero),
            Symbol::Five => (Symbol::Six, Symbol::Zero),
            Symbol::Six => (Symbol::Seven, Symbol::Zero),
            Symbol::Seven => (Symbol::Eight, Symbol::Zero),
            Symbol::Eight => (Symbol::Nine, Symbol::Zero),
            Symbol::Nine => (Symbol::Zero, Symbol::One),
        },
        Symbol::Two => match b {
            Symbol::Zero => (Symbol::Two, Symbol::Zero),
            Symbol::One => (Symbol::Three, Symbol::Zero),
            Symbol::Two => (Symbol::Four, Symbol::Zero),
            Symbol::Three => (Symbol::Five, Symbol::Zero),
            Symbol::Four => (Symbol::Six, Symbol::Zero),
            Symbol::Five => (Symbol::Seven, Symbol::Zero),
            Symbol::Six => (Symbol::Eight, Symbol::Zero),
            Symbol::Seven => (Symbol::Nine, Symbol::Zero),
            Symbol::Eight => (Symbol::Zero, Symbol::One),
            Symbol::Nine => (Symbol::One, Symbol::One),
        },
        Symbol::Three => match b {
            Symbol::Zero => (Symbol::Three, Symbol::Zero),
            Symbol::One => (Symbol::Four, Symbol::Zero),
            Symbol::Two => (Symbol::Five, Symbol::Zero),
            Symbol::Three => (Symbol::Six, Symbol::Zero),
            Symbol::Four => (Symbol::Seven, Symbol::Zero),
            Symbol::Five => (Symbol::Eight, Symbol::Zero),
            Symbol::Six => (Symbol::Nine, Symbol::Zero),
            Symbol::Seven => (Symbol::Zero, Symbol::One),
            Symbol::Eight => (Symbol::One, Symbol::One),
            Symbol::Nine => (Symbol::Two, Symbol::One),
        },
        Symbol::Four => match b {
            Symbol::Zero => (Symbol::Four, Symbol::Zero),
            Symbol::One => (Symbol::Five, Symbol::Zero),
            Symbol::Two => (Symbol::Six, Symbol::Zero),
            Symbol::Three => (Symbol::Seven, Symbol::Zero),
            Symbol::Four => (Symbol::Eight, Symbol::Zero),
            Symbol::Five => (Symbol::Nine, Symbol::Zero),
            Symbol::Six => (Symbol::Zero, Symbol::One),
            Symbol::Seven => (Symbol::One, Symbol::One),
            Symbol::Eight => (Symbol::Two, Symbol::One),
            Symbol::Nine => (Symbol::Three, Symbol::One),
        },
        Symbol::Five => match b {
            Symbol::Zero => (Symbol::Five, Symbol::Zero),
            Symbol::One => (Symbol::Six, Symbol::Zero),
            Symbol::Two => (Symbol::Seven, Symbol::Zero),
            Symbol::Three => (Symbol::Eight, Symbol::Zero),
            Symbol::Four => (Symbol::Nine, Symbol::Zero),
            Symbol::Five => (Symbol::Zero, Symbol::One),
            Symbol::Six => (Symbol::One, Symbol::One),
            Symbol::Seven => (Symbol::Two, Symbol::One),
            Symbol::Eight => (Symbol::Three, Symbol::One),
            Symbol::Nine => (Symbol::Four, Symbol::One),
        },
        Symbol::Six => match b {
            Symbol::Zero => (Symbol::Six, Symbol::Zero),
            Symbol::One => (Symbol::Seven, Symbol::Zero),
            Symbol::Two => (Symbol::Eight, Symbol::Zero),
            Symbol::Three => (Symbol::Nine, Symbol::Zero),
            Symbol::Four => (Symbol::Zero, Symbol::One),
            Symbol::Five => (Symbol::One, Symbol::One),
            Symbol::Six => (Symbol::Two, Symbol::One),
            Symbol::Seven => (Symbol::Three, Symbol::One),
            Symbol::Eight => (Symbol::Four, Symbol::One),
            Symbol::Nine => (Symbol::Five, Symbol::One),
        },
        Symbol::Seven => match b {
            Symbol::Zero => (Symbol::Seven, Symbol::Zero),
            Symbol::One => (Symbol::Eight, Symbol::Zero),
            Symbol::Two => (Symbol::Nine, Symbol::Zero),
            Symbol::Three => (Symbol::Zero, Symbol::One),
            Symbol::Four => (Symbol::One, Symbol::One),
            Symbol::Five => (Symbol::Two, Symbol::One),
            Symbol::Six => (Symbol::Three, Symbol::One),
            Symbol::Seven => (Symbol::Four, Symbol::One),
            Symbol::Eight => (Symbol::Five, Symbol::One),
            Symbol::Nine => (Symbol::Six, Symbol::One),
        },
        Symbol::Eight => match b {
            Symbol::Zero => (Symbol::Eight, Symbol::Zero),
            Symbol::One => (Symbol::Nine, Symbol::Zero),
            Symbol::Two => (Symbol::Zero, Symbol::One),
            Symbol::Three => (Symbol::One, Symbol::One),
            Symbol::Four => (Symbol::Two, Symbol::One),
            Symbol::Five => (Symbol::Three, Symbol::One),
            Symbol::Six => (Symbol::Four, Symbol::One),
            Symbol::Seven => (Symbol::Five, Symbol::One),
            Symbol::Eight => (Symbol::Six, Symbol::One),
            Symbol::Nine => (Symbol::Seven, Symbol::One),
        },
        Symbol::Nine => match b {
            Symbol::Zero => (Symbol::Nine, Symbol::Zero),
            Symbol::One => (Symbol::Zero, Symbol::One),
            Symbol::Two => (Symbol::One, Symbol::One),
            Symbol::Three => (Symbol::Two, Symbol::One),
            Symbol::Four => (Symbol::Three, Symbol::One),
            Symbol::Five => (Symbol::Four, Symbol::One),
            Symbol::Six => (Symbol::Five, Symbol::One),
            Symbol::Seven => (Symbol::Six, Symbol::One),
            Symbol::Eight => (Symbol::Seven, Symbol::One),
            Symbol::Nine => (Symbol::Eight, Symbol::One),
        },
    }
}

use std::ops::Add;
impl Add for UnsignedInteger {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let frame = self
            .symbols
            .iter()
            .zip(other.symbols.iter())
            .map(|(a, b)| half_add_unit(a, b))
            .collect::<Vec<_>>();
        let units = frame.iter().cloned().map(|(u, _)| u);
        let tens = frame.iter().cloned().map(|(_, t)| t);
        let done = tens.clone().all(|x| x == Symbol::Zero);
        if done {
            UnsignedInteger::raw(units.collect::<Vec<_>>().as_slice())
        } else {
            //add a leading zero to units
            let new_units = {
                use std::iter;
                let once = iter::once(Symbol::Zero);
                units.chain(once).collect::<Vec<_>>()
            };

            //add a zero to the units of the tens to push it up by one
            let new_tens = {
                use std::iter;
                let once = iter::once(Symbol::Zero);
                once.chain(tens).collect::<Vec<_>>()
            };
            UnsignedInteger::raw(&new_units) + UnsignedInteger::raw(&new_tens)
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_interger_from_str_0() {
        assert_eq!(
            UnsignedInteger::from_str("0"),
            Ok(UnsignedInteger::raw(&[Symbol::Zero]))
        );
    }
    #[test]
    fn test_interger_from_str_1() {
        assert_eq!(
            UnsignedInteger::from_str("1"),
            Ok(UnsignedInteger::raw(&[Symbol::One]))
        );
    }
    #[test]
    fn test_interger_from_str_2() {
        assert_eq!(
            UnsignedInteger::from_str("2"),
            Ok(UnsignedInteger::raw(&[Symbol::Two]))
        );
    }
    #[test]
    fn test_interger_from_str_3() {
        assert_eq!(
            UnsignedInteger::from_str("3"),
            Ok(UnsignedInteger::raw(&[Symbol::Three]))
        );
    }
    #[test]
    fn test_interger_from_str_4() {
        assert_eq!(
            UnsignedInteger::from_str("4"),
            Ok(UnsignedInteger::raw(&[Symbol::Four]))
        );
    }
    #[test]
    fn test_interger_from_str_5() {
        assert_eq!(
            UnsignedInteger::from_str("5"),
            Ok(UnsignedInteger::raw(&[Symbol::Five]))
        );
    }
    #[test]
    fn test_interger_from_str_6() {
        assert_eq!(
            UnsignedInteger::from_str("6"),
            Ok(UnsignedInteger::raw(&[Symbol::Six]))
        );
    }
    #[test]
    fn test_interger_from_str_7() {
        assert_eq!(
            UnsignedInteger::from_str("7"),
            Ok(UnsignedInteger::raw(&[Symbol::Seven]))
        );
    }
    #[test]
    fn test_interger_from_str_8() {
        assert_eq!(
            UnsignedInteger::from_str("8"),
            Ok(UnsignedInteger::raw(&[Symbol::Eight]))
        );
    }
    #[test]
    fn test_interger_from_str_9() {
        assert_eq!(
            UnsignedInteger::from_str("9"),
            Ok(UnsignedInteger::raw(&[Symbol::Nine]))
        );
    }
    #[test]
    fn test_interger_from_str_10() {
        assert_eq!(
            UnsignedInteger::from_str("10"),
            Ok(UnsignedInteger::raw(&[Symbol::Zero, Symbol::One]))
        );
    }
    #[test]
    fn test_interger_from_str_empty() {
        assert_eq!(
            UnsignedInteger::from_str(""),
            Err(ParseUnsignedIntegerError::EmptyString)
        );
    }
    #[test]
    fn test_interger_from_str_not_a_number() {
        assert_eq!(
            UnsignedInteger::from_str("Hello World"),
            Err(ParseUnsignedIntegerError::NotANumber)
        );
    }
    #[test]
    fn test_interger_from_str_minus_1() {
        assert_eq!(
            UnsignedInteger::from_str("-1"),
            Err(ParseUnsignedIntegerError::NotANumber)
        );
    }
    #[test]
    fn test_interger_from_str_minus_10() {
        assert_eq!(
            UnsignedInteger::from_str("-10"),
            Err(ParseUnsignedIntegerError::NotANumber)
        );
    }
    #[test]
    fn test_interger_from_str_plus_10() {
        assert_eq!(
            UnsignedInteger::from_str("+10"),
            Err(ParseUnsignedIntegerError::NotANumber)
        );
    }
    #[test]
    fn test_interger_from_str_minus_000010() {
        assert_eq!(
            UnsignedInteger::from_str("-000010"),
            Err(ParseUnsignedIntegerError::NotANumber)
        );
    }
    #[test]
    fn test_interger_from_str_plus_000010() {
        assert_eq!(
            UnsignedInteger::from_str("+000010"),
            Err(ParseUnsignedIntegerError::NotANumber)
        );
    }
    #[test]
    fn test_interger_from_str_minus_0() {
        assert_eq!(
            UnsignedInteger::from_str("-0"),
            Err(ParseUnsignedIntegerError::NotANumber)
        );
    }
    #[test]
    fn test_interger_from_str_plus_0() {
        assert_eq!(
            UnsignedInteger::from_str("+0"),
            Err(ParseUnsignedIntegerError::NotANumber)
        );
    }
    #[test]
    fn test_interger_from_str_plus_00000() {
        assert_eq!(
            UnsignedInteger::from_str("+00000"),
            Err(ParseUnsignedIntegerError::NotANumber)
        );
    }
    #[test]
    fn test_interger_from_str_dashes_centre() {
        assert_eq!(
            UnsignedInteger::from_str("125-12"),
            Err(ParseUnsignedIntegerError::NotANumber)
        );
    }
    #[test]
    fn test_interger_from_str_pluses_centre() {
        assert_eq!(
            UnsignedInteger::from_str("125+12"),
            Err(ParseUnsignedIntegerError::NotANumber)
        );
    }

    #[test]
    fn test_display() {
        assert_eq!(
            format!("{}", UnsignedInteger::from_str("12512").unwrap()),
            "12512".to_string()
        );
    }

    #[test]
    fn test_add_single_digits() {
        let digits = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]
            .iter()
            .map(|x| {
                (
                    UnsignedInteger::from_str(x).unwrap(),
                    u32::from_str(x).unwrap(),
                )
            });
        use itertools::Itertools;
        digits
            .clone()
            .cartesian_product(digits)
            .map(|((big_x, x), (big_y, y))| (big_x + big_y, x + y))
            .map(|(big, little)| (format!("{}", big), format!("{}", little)))
            .for_each(|(result, expected)| assert_eq!(result, expected));
    }

    fn test_add() {
        let cases = ["0", "12", "15", "50", "75", "128", "613", "1024", "4221", "7555"]
            .iter()
            .map(|x| {
                (
                    UnsignedInteger::from_str(x).unwrap(),
                    u32::from_str(x).unwrap(),
                )
            });
        use itertools::Itertools;
        cases
            .clone()
            .cartesian_product(cases)
            .map(|((big_x, x), (big_y, y))| (big_x + big_y, x + y))
            .map(|(big, little)| (format!("{}", big), format!("{}", little)))
            .for_each(|(result, expected)| assert_eq!(result, expected));
    }
}
