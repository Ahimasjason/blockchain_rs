use std::cmp::PartialEq;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Copy, Clone)]
pub struct FieldElement {
    num: usize,
    prime: usize,
}

impl FieldElement {
    pub fn new(num: usize, prime: usize) -> Result<Self, String> {
        if num >= prime {
            let msg = format!("Num {} not in field range 0 to {}", num, prime - 1);
            return Err(msg);
        }
        Ok(Self { num, prime })
    }
}

impl Add for FieldElement {
    type Output = Result<Self, String>;

    fn add(self, other: Self) -> Self::Output {
        if other.prime != self.prime {
            return Err(format!("Cannot add two nums in different fields"));
        }

        Self::new((self.num + other.num) % self.prime, self.prime)
    }
}

impl Sub for FieldElement {
    type Output = Result<Self, String>;

    fn sub(self, other: Self) -> Self::Output {
        if other.prime != self.prime {
            return Err(format!("Cannot add two nums in different fields"));
        }
        let num: isize =
            ((self.num as isize - other.num as isize) as isize).rem_euclid(self.prime as isize);
        Self::new(num as usize, self.prime)
    }
}

impl PartialEq for FieldElement {
    fn eq(&self, other: &Self) -> bool {
        (self.num == other.num) && (self.prime == other.prime)
    }
}

impl Mul for FieldElement {
    type Output = Result<Self, String>;

    fn mul(self, other: Self) -> Self::Output {
        if other.prime != self.prime {
            return Err(format!("Cannot add two nums in different fields"));
        }

        Self::new((other.num * self.num).rem_euclid(self.prime), self.prime)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let a = FieldElement::new(7, 13).unwrap();
        assert_eq!(&a, &a);
    }

    #[test]
    fn ne_works() {
        let a = FieldElement::new(7, 13).unwrap();
        let b = FieldElement::new(7, 17).unwrap();
        assert_ne!(&a, &b);
    }

    #[test]
    fn test_add() {
        let a = FieldElement::new(7, 13).unwrap();
        let b = FieldElement::new(12, 13).unwrap();
        let c = FieldElement::new(6, 13).unwrap();
        assert_eq!((a + b).unwrap(), c);
    }

    #[test]
    fn test_sub() {
        let a = FieldElement::new(7, 13).unwrap();
        let b = FieldElement::new(12, 13).unwrap();
        let c = FieldElement::new(8, 13).unwrap();
        assert_eq!((a - b).unwrap(), c);
    }

    #[test]
    fn test_mul() {
        let a = FieldElement::new(3, 13).unwrap();
        let b = FieldElement::new(12, 13).unwrap();
        let c = FieldElement::new(10, 13).unwrap();
        assert_eq!((a * b).unwrap(), c);
    }
}
