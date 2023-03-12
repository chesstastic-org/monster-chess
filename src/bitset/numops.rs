use super::BitSet;
use std::ops::{Add, AddAssign, Sub, SubAssign};

impl<const T: usize> Add<BitSet<T>> for BitSet<T> {
    type Output = Self;

    fn add(self, rhs: BitSet<T>) -> BitSet<T> {
        let mut bitset = self.clone();
        bitset += rhs;
        bitset
    }
}

impl<const T: usize> AddAssign<BitSet<T>> for BitSet<T> {
    fn add_assign(&mut self, rhs: BitSet<T>) {
        if T == 1 {
            self.data[0] += rhs.data[0];
            return;
        }

        let mut carry = *self & rhs;
        *self ^= rhs;
        while carry.is_set() {
            let shifted_carry = carry << 1;
            carry = *self & shifted_carry;
            *self ^= shifted_carry;
        }
    }
}

impl<const T: usize> Sub<BitSet<T>> for BitSet<T> {
    type Output = Self;

    fn sub(self, rhs: BitSet<T>) -> BitSet<T> {
        let mut bitset = self.clone();
        bitset -= rhs;
        bitset
    }
}

impl<const T: usize> SubAssign<BitSet<T>> for BitSet<T> {
    fn sub_assign(&mut self, rhs: BitSet<T>) {
        if T == 1 {
            self.data[0] -= rhs.data[0];
            return;
        }

        let mut rhs = rhs.clone();
        while rhs.is_set() {
            let borrow = !(*self) & rhs;
            *self ^= rhs;
            rhs = borrow << 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BitSet;

    #[test]
    fn add() {
        assert_eq!(
            BitSet::from_data([2, 13]) + &BitSet::from_data([4, 19]),
            BitSet::from_data([6, 32])
        );
        assert_eq!(
            BitSet::from_data([0, u128::MAX]) + &BitSet::from_data([0, 1]),
            BitSet::from_data([1, 0])
        );
    }

    #[test]
    fn sub() {
        assert_eq!(
            BitSet::from_data([6, 32]) - &BitSet::from_data([4, 19]),
            BitSet::from_data([2, 13])
        );
        assert_eq!(
            BitSet::from_data([1, 0]) - &BitSet::from_data([0, 1]),
            BitSet::from_data([0, u128::MAX])
        );
    }
}
