use super::BitBoard;
use std::ops::{Add, AddAssign, Sub, SubAssign};

impl<const T: usize> Add<BitBoard<T>> for BitBoard<T> {
    type Output = Self;

    fn add(self, rhs: BitBoard<T>) -> BitBoard<T> {
        let mut BitBoard = self.clone();
        BitBoard += rhs;
        BitBoard
    }
}

impl<const T: usize> AddAssign<BitBoard<T>> for BitBoard<T> {
    fn add_assign(&mut self, rhs: BitBoard<T>) {
        if T == 1 {
            self.bits[0] += rhs.bits[0];
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

impl<const T: usize> Sub<BitBoard<T>> for BitBoard<T> {
    type Output = Self;

    fn sub(self, rhs: BitBoard<T>) -> BitBoard<T> {
        let mut BitBoard = self.clone();
        BitBoard -= rhs;
        BitBoard
    }
}

impl<const T: usize> SubAssign<BitBoard<T>> for BitBoard<T> {
    fn sub_assign(&mut self, rhs: BitBoard<T>) {
        if T == 1 {
            self.bits[0] -= rhs.bits[0];
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
    use super::BitBoard;

    #[test]
    fn add() {
        assert_eq!(
            BitBoard::from_data([2, 13]) + BitBoard::from_data([4, 19]),
            BitBoard::from_data([6, 32])
        );
        assert_eq!(
            BitBoard::from_data([0, u128::MAX]) + BitBoard::from_data([0, 1]),
            BitBoard::from_data([1, 0])
        );
    }

    #[test]
    fn sub() {
        assert_eq!(
            BitBoard::from_data([6, 32]) - BitBoard::from_data([4, 19]),
            BitBoard::from_data([2, 13])
        );
        assert_eq!(
            BitBoard::from_data([1, 0]) - BitBoard::from_data([0, 1]),
            BitBoard::from_data([0, u128::MAX])
        );
    }
}
