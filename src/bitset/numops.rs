use crate::BitSet;
use std::ops::{AddAssign, Add, Sub, SubAssign};

impl<const T: usize> Add<&BitSet<T>> for BitSet<T> {
    type Output = Self;

    fn add(self, rhs: &BitSet<T>) -> BitSet<T> {
        let mut bitset = self.clone();
        bitset += rhs;
        bitset
    }
}

impl<const T: usize> AddAssign<&BitSet<T>> for BitSet<T> {
    fn add_assign(&mut self, rhs: &BitSet<T>) {
        if T == 1 {
            self.data[0] += rhs.data[0];
            return;
        }

        let mut carry = *self & rhs;
        *self ^= rhs;
        while !carry.is_empty() {
            let shifted_carry = carry << 1;
            carry = *self & &shifted_carry;
            *self ^= &shifted_carry;
        }
    }
}


impl<const T: usize> Sub<&BitSet<T>> for BitSet<T> {
    type Output = Self;

    fn sub(self, rhs: &BitSet<T>) -> BitSet<T> {
        let mut bitset = self.clone();
        bitset -= rhs;
        bitset
    }
}

impl<const T: usize> SubAssign<&BitSet<T>> for BitSet<T> {
    fn sub_assign(&mut self, rhs: &BitSet<T>) {
        if T == 1 {
            self.data[0] -= rhs.data[0];
            return;
        }

        let mut rhs = rhs.clone();
        while !rhs.is_empty() {
            let borrow = !(*self) & &rhs;
            *self ^= &rhs;
            rhs = borrow << 1;
        }
    }
}