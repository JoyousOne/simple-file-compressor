use std::fmt;

type Bit = u8;

#[derive(Debug)]
pub struct BigNum {
    pub bits: Vec<u8>,
}

impl Clone for BigNum {
    fn clone(&self) -> Self {
        BigNum {
            bits: self.bits.clone(),
        }
    }
}

impl BigNum {
    pub fn new() -> Self {
        BigNum { bits: Vec::new() }
    }

    pub fn push_bit(&mut self, bit: Bit) {
        assert!(bit == 0 || bit == 1);
        self.bits.push(bit);
    }

    // fn _add(&mut self, other: &Self) {}

    pub fn add(&mut self, other: &Self) {
        self.align_to_other(other);
        let other = self.get_aligned_num(other);

        let mut carry = 0;
        for i in (0..self.bits.len()).rev() {
            let total = self.bits[i] + other.bits[i] + carry;

            self.bits[i] = total % 2;
            carry = total / 2;
        }

        if carry > 0 {
            self.bits.insert(0, carry);
        }
    }

    pub fn substract(&mut self, other: &Self) {}

    pub fn multiply(&mut self, other: &Self) {
        // Handle cases where one of the values is 0
        if self.bits == vec![0] {
            return;
        } else if other.bits == vec![0] {
            self.bits = vec![0];
            return;
        }

        let mut total = BigNum::new();
        for (offset, &muliplier) in other.bits.iter().rev().enumerate() {
            if muliplier == 0 {
                continue;
            }

            // generate partial_product
            let mut partial_product = BigNum::new();
            for &multiplicand in &self.bits {
                if multiplicand == 1 {
                    partial_product.push_bit(1);
                } else {
                    partial_product.push_bit(0);
                }
            }

            // add the placeholder that will shift our value
            for _ in 0..offset {
                partial_product.push_bit(0);
            }

            total.add(&partial_product);
        }

        *self = total;
    }

    fn align_to_other(&mut self, other: &Self) {
        if self.bits.len() < other.bits.len() {
            let offset = other.bits.len() - self.bits.len();
            let mut new_bits = vec![0; offset];
            new_bits.extend_from_slice(&self.bits);
            self.bits = new_bits;
        }
    }

    fn get_aligned_num(&self, other: &Self) -> Self {
        if self.bits.len() > other.bits.len() {
            let offset = self.bits.len() - other.bits.len();
            let mut new_bits = vec![0; offset];
            new_bits.extend_from_slice(&other.bits);
            // self.bits = new_bits;
            BigNum::from(&new_bits)
        } else {
            other.clone()
        }
    }

    pub fn print(&self) {
        let decimal = DecimalNum::from(&self.bits);

        for digit in decimal.digits {
            print!("{digit}");
        }
        println!()
    }
}

impl From<usize> for BigNum {
    fn from(value: usize) -> Self {
        let size_of_usize = std::mem::size_of::<usize>();
        let mut bits = Vec::with_capacity(size_of_usize);

        for i in 0..size_of_usize {
            let bit = (value >> i) & 1;
            bits.push(bit as u8);
        }

        bits.reverse();

        BigNum { bits }
    }
}

impl From<&usize> for BigNum {
    fn from(value: &usize) -> Self {
        (*value).into()
    }
}
impl From<&Vec<Bit>> for BigNum {
    fn from(value: &Vec<Bit>) -> Self {
        BigNum {
            bits: value.to_vec(),
        }
    }
}

impl fmt::Display for BigNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let decimal = DecimalNum::from(&self.bits);
        let mut str = String::new();
        for digit in decimal.digits {
            str.push((digit + b'0') as char);
        }
        write!(f, "{}", &str)
    }
}

pub struct DecimalNum {
    pub digits: Vec<u8>,
}

impl DecimalNum {
    pub fn new() -> Self {
        DecimalNum { digits: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        DecimalNum {
            digits: Vec::with_capacity(capacity),
        }
    }

    fn multiply_by_two(&mut self) {
        if self.digits.len() == 0 || self.digits[0] != 0 {
            self.digits.insert(0, 0);
        }

        let mut carry = 0;

        for i in (0..self.digits.len()).rev() {
            let digit = &mut self.digits[i];
            let total = *digit * 2 + carry;

            *digit = total % 10;
            carry = total / 10;
        }
    }

    fn add_bit(&mut self, bit: Bit) {
        // handle first added bit
        if self.digits.is_empty() {
            self.digits.push(bit);
        }

        let mut carry = bit;
        for i in (0..self.digits.len()).rev() {
            let digit = &mut self.digits[i];
            let total = *digit + carry;

            *digit = total % 10;
            carry = total / 10;

            if carry == 0 {
                break;
            }
        }
    }
}

impl From<&Vec<Bit>> for DecimalNum {
    fn from(bits: &Vec<Bit>) -> Self {
        let bin_length = bits.len();
        let num_digit = (bin_length / 2) + (bin_length % 2);
        let mut decimal_num = DecimalNum::with_capacity(num_digit);

        for &bit in bits {
            decimal_num.multiply_by_two();
            decimal_num.add_bit(bit);
        }

        if decimal_num.digits.len() > 1 && decimal_num.digits[0] == 0 {
            decimal_num.digits.remove(0);
        }

        decimal_num
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_bits_to_decimal() {
        let num: Vec<Bit> = vec![0];
        let dec_num = DecimalNum::from(&num);
        assert_eq!(vec![0], dec_num.digits);

        let num: Vec<Bit> = vec![1];
        let dec_num = DecimalNum::from(&num);
        assert_eq!(vec![1], dec_num.digits);

        let num: Vec<Bit> = vec![1, 0, 1, 0, 1];
        let dec_num = DecimalNum::from(&num);
        assert_eq!(vec![2, 1], dec_num.digits);

        // MAX
        let num: Vec<Bit> = vec![1; 128];
        let dec_num = DecimalNum::from(&num);
        assert_eq!(
            vec![
                3, 4, 0, 2, 8, 2, 3, 6, 6, 9, 2, 0, 9, 3, 8, 4, 6, 3, 4, 6, 3, 3, 7, 4, 6, 0, 7, 4,
                3, 1, 7, 6, 8, 2, 1, 1, 4, 5, 5
            ],
            dec_num.digits
        );

        // BEYOND MAX
        let mut num: Vec<Bit> = vec![0; 129];
        num[0] = 1;
        let dec_num = DecimalNum::from(&num);
        assert_eq!(
            vec![
                3, 4, 0, 2, 8, 2, 3, 6, 6, 9, 2, 0, 9, 3, 8, 4, 6, 3, 4, 6, 3, 3, 7, 4, 6, 0, 7, 4,
                3, 1, 7, 6, 8, 2, 1, 1, 4, 5, 6
            ],
            dec_num.digits
        );
    }

    #[test]
    fn bignum_addition() {
        let mut num_a = BigNum::from(&vec![0]);
        let num_b = BigNum::from(&vec![0]);
        num_a.add(&num_b);
        assert_eq!(vec![0], num_a.bits);

        let mut num_a = BigNum::from(&vec![0]);
        let num_b = BigNum::from(&vec![1]);
        num_a.add(&num_b);
        assert_eq!(vec![1], num_a.bits);

        let mut num_a = BigNum::from(&vec![1]);
        let num_b = BigNum::from(&vec![1]);
        num_a.add(&num_b);
        assert_eq!(vec![1, 0], num_a.bits);

        let mut num_a = BigNum::from(&vec![1, 0]);
        let num_b = BigNum::from(&vec![1]);
        num_a.add(&num_b);
        assert_eq!(vec![1, 1], num_a.bits);

        let mut num_a = BigNum::from(&vec![1]);
        let num_b = BigNum::from(&vec![1, 0]);
        num_a.add(&num_b);
        assert_eq!(vec![1, 1], num_a.bits);

        let mut num_a = BigNum::from(&vec![1, 0, 1, 0, 1]);
        let num_b = BigNum::from(&vec![1, 0, 1, 0, 1]);
        num_a.add(&num_b);
        assert_eq!(vec![1, 0, 1, 0, 1, 0], num_a.bits);
    }

    #[test]
    fn bignum_multiply() {
        // CASE WHERE RESULT IS 0
        let mut num_a = BigNum::from(&vec![0]);
        let num_b = BigNum::from(&vec![0]);
        num_a.multiply(&num_b);
        assert_eq!(vec![0], num_a.bits);

        let mut num_a = BigNum::from(&vec![0]);
        let num_b = BigNum::from(&vec![1]);
        num_a.multiply(&num_b);
        assert_eq!(vec![0], num_a.bits);

        let mut num_a = BigNum::from(&vec![1]);
        let num_b = BigNum::from(&vec![0]);
        num_a.multiply(&num_b);
        assert_eq!(vec![0], num_a.bits);
        // END CASE 0

        let mut num_a = BigNum::from(&vec![1]);
        let num_b = BigNum::from(&vec![1]);
        num_a.multiply(&num_b);
        assert_eq!(vec![1], num_a.bits);

        let mut num_a = BigNum::from(&vec![1, 0]);
        let num_b = BigNum::from(&vec![1, 0]);
        num_a.multiply(&num_b);
        assert_eq!(vec![1, 0, 0], num_a.bits);

        let mut num_a = BigNum::from(&vec![1]);
        let num_b = BigNum::from(&vec![1, 0]);
        num_a.multiply(&num_b);
        assert_eq!(vec![1, 0], num_a.bits);

        let mut num_a = BigNum::from(&vec![1, 0, 0, 0, 1]);
        let num_b = BigNum::from(&vec![1, 0, 1, 0]);
        num_a.multiply(&num_b);
        assert_eq!(vec![1, 0, 1, 0, 1, 0, 1, 0], num_a.bits);
    }
}
