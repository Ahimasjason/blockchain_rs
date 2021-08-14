use std::cmp::PartialEq;
use std::ops::{Add, Div, Mul, Sub};

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

    pub fn pow(&self, power: i64) -> Result<Self, String> {
        // let mut n = power;
        // while n < 0{
        //     n += (self.prime - 1) as i32;

        // }
        let n = power.rem_euclid((self.prime - 1) as i64);
        let num = (self.num as i64)
            .pow(n as u32)
            .rem_euclid(self.prime as i64);
        Self::new(num as usize, self.prime)
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

impl Div for FieldElement {
    type Output = Result<Self, String>;

    fn div(self, other: Self) -> Self::Output {
        if other.prime != self.prime {
            return Err(format!("Cannot add two nums in different fields"));
        }
        let n = pow_mod(other.num, self.prime - 2, self.prime);
        let num = (self.num * n) % self.prime;
        Self::new(num, self.prime)
    }
}


//  pow_mod from stack overflow for handling usize overflow caused by u32.pow
fn pow_mod(mut a:usize, mut b:usize, modulo: usize) -> usize{
    
    let mut num = 1;
    // let mut b= b;
    while b > 0{
        if b & 1 != 0 {
           num = (num * a).rem_euclid(modulo); 
        }
        
        b >>= 1;
        a = (a*a).rem_euclid(modulo);
    }
    num
    
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

    #[test]
    fn test_pow() {
        let a = FieldElement::new(3, 13).unwrap();
        // let b = FieldElement::new(12, 13).unwrap();
        let c = FieldElement::new(1, 13).unwrap();
        assert_eq!(a.pow(3).unwrap(), c);
        let d = FieldElement::new(7, 13).unwrap();
        let e = FieldElement::new(8, 13).unwrap();
        assert_eq!(d.pow(-3).unwrap(), e);

        // print(a**-3==b)
    }

    #[test]
    fn test_div() {
        let a = FieldElement::new(3, 31).unwrap();
        let b = FieldElement::new(4, 31).unwrap();
        let c = FieldElement::new(24, 31).unwrap();
        // let h = a / c;
        assert_eq!((a / c).unwrap(), b);
    }
}
