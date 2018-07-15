use std::fmt::Display;
use std::str::FromStr;
use std::cmp::Ordering;

use bigdecimal::{BigDecimal, ParseBigDecimalError};
use num_traits::{Zero, One};
use std::ops::{Add, Mul};

#[derive(Debug, Clone, Hash)]
pub enum BigFloat {
    Decimal(BigDecimal),
    PlusInfinity,
    MinusInfinity,
    NaN,
}
impl FromStr for BigFloat {
    type Err = ParseBigDecimalError;

    /// https://www.w3.org/TR/xmlschema11-2/#sec-lex-float
    ///
    /// Make sure to remove all whitespaces before calling this.
    fn from_str(s: &str) -> Result<BigFloat, ParseBigDecimalError> {
        match s {
            "INF" | "+INF" => Ok(BigFloat::PlusInfinity),
            "-INF" => Ok(BigFloat::MinusInfinity),
            "NaN" => Ok(BigFloat::NaN),
            _ => BigDecimal::from_str(s).map(BigFloat::Decimal),
        }
    }
}
impl PartialEq for BigFloat {
    fn eq(&self, rhs: &BigFloat) -> bool {
        use self::BigFloat::*;
        match (self, rhs) {
            (NaN, _) | (_, NaN) => false,
            (PlusInfinity, PlusInfinity) => true,
            (MinusInfinity, MinusInfinity) => true,
            (Decimal(l), Decimal(r)) => l == r,
            _ => false,
        }
    }
}
impl PartialOrd for BigFloat {
    fn partial_cmp(&self, rhs: &BigFloat) -> Option<Ordering> {
        use self::BigFloat::*;
        if self == rhs {
            Some(Ordering::Equal)
        }
        else {
            match (self, rhs) {
                (NaN, _) | (_, NaN) => None,
                (PlusInfinity, _) => Some(Ordering::Less),
                (MinusInfinity, _) => Some(Ordering::Greater),
                (_, PlusInfinity) => Some(Ordering::Greater),
                (_, MinusInfinity) => Some(Ordering::Less),
                (Decimal(l), Decimal(r)) => l.partial_cmp(r),
            }
        }
    }
}
impl Display for BigFloat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
        use self::BigFloat::*;
        match self {
            NaN => write!(f, "NaN"),
            PlusInfinity => write!(f, "+INF"), // TODO: what's the canonical repr?
            MinusInfinity => write!(f, "-INF"),
            Decimal(d) => d.fmt(f),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, PartialOrd)]
pub struct BigFloatNotNaN(BigFloat);
impl Eq for BigFloatNotNaN {}
impl Ord for BigFloatNotNaN {
    fn cmp(&self, rhs: &BigFloatNotNaN) -> Ordering {
        self.0.partial_cmp(&rhs.0).expect("NaN")
    }
}
impl From<BigDecimal> for BigFloatNotNaN {
    fn from(d: BigDecimal) -> BigFloatNotNaN {
        BigFloatNotNaN(BigFloat::Decimal(d))
    }
}
impl From<BigFloat> for BigFloatNotNaN {
    fn from(f: BigFloat) -> BigFloatNotNaN {
        if f == BigFloat::NaN {
            panic!("NaN");
        }
        BigFloatNotNaN(f)
    }
}
impl FromStr for BigFloatNotNaN {
    type Err = ParseBigDecimalError;

    /// https://www.w3.org/TR/xmlschema11-2/#sec-lex-float
    ///
    /// Make sure to remove all whitespaces before calling this.
    fn from_str(s: &str) -> Result<BigFloatNotNaN, ParseBigDecimalError> {
        match s {
            "INF" | "+INF" => Ok(BigFloatNotNaN(BigFloat::PlusInfinity)),
            "-INF" => Ok(BigFloatNotNaN(BigFloat::MinusInfinity)),
            "NaN" => Err(ParseBigDecimalError::Other("NaN".to_string())),
            _ => Ok(BigFloatNotNaN::from(BigDecimal::from_str(s)?)),
        }
    }
}
impl Add for BigFloatNotNaN {
    type Output = BigFloatNotNaN;
    fn add(self, rhs: BigFloatNotNaN) -> BigFloatNotNaN {
        use self::BigFloat::*;
        match (self.0, rhs.0) {
            (NaN, _) | (_, NaN) => panic!("NaN"),
            (PlusInfinity, MinusInfinity) => panic!("+inf + -inf"),
            (MinusInfinity, PlusInfinity) => panic!("-inf + +inf"),
            (PlusInfinity, _) | (_, PlusInfinity) => BigFloatNotNaN(PlusInfinity),
            (MinusInfinity, _) | (_, MinusInfinity) => BigFloatNotNaN(MinusInfinity),
            (Decimal(f), Decimal(r)) => BigFloatNotNaN(Decimal(f + r)),
        }
    }
}
impl Mul for BigFloatNotNaN {
    type Output = BigFloatNotNaN;
    fn mul(self, rhs: BigFloatNotNaN) -> BigFloatNotNaN {
        use self::BigFloat::*;
        match (self.0, rhs.0) {
            (NaN, _) | (_, NaN) => panic!("NaN"),
            (PlusInfinity, MinusInfinity) | (MinusInfinity, PlusInfinity) => BigFloatNotNaN(MinusInfinity),
            (PlusInfinity, PlusInfinity) | (MinusInfinity, MinusInfinity) => BigFloatNotNaN(PlusInfinity),
            (PlusInfinity, Decimal(d)) | (Decimal(d), PlusInfinity) => {
                if BigDecimal::is_zero(&d) {
                    panic!("+inf * 0")
                }
                else if d > BigDecimal::zero() {
                    BigFloatNotNaN(PlusInfinity)
                }
                else {
                    BigFloatNotNaN(MinusInfinity)
                }
            },
            (MinusInfinity, Decimal(d)) | (Decimal(d), MinusInfinity) => {
                if BigDecimal::is_zero(&d) {
                    panic!("-inf * 0")
                }
                else if d > BigDecimal::zero() {
                    BigFloatNotNaN(MinusInfinity)
                }
                else {
                    BigFloatNotNaN(PlusInfinity)
                }
            },
            (Decimal(l), Decimal(r)) => BigFloatNotNaN(Decimal(l * r)),
        }
    }
}
impl Zero for BigFloatNotNaN {
    fn zero() -> BigFloatNotNaN {
        BigFloatNotNaN(BigFloat::Decimal(BigDecimal::zero()))
    }
    fn is_zero(&self) -> bool {
        match self {
            BigFloatNotNaN(BigFloat::Decimal(d)) => d.is_zero(),
            _ => false,
        }
    }
}
impl One for BigFloatNotNaN {
    fn one() -> BigFloatNotNaN {
        BigFloatNotNaN(BigFloat::Decimal(BigDecimal::one()))
    }
    fn is_one(&self) -> bool {
        match self {
            BigFloatNotNaN(BigFloat::Decimal(d)) => d.is_one(),
            _ => false,
        }
    }
}
impl Display for BigFloatNotNaN {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
        self.0.fmt(f)
    }
}

