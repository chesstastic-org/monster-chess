use super::BitBoard;
use std::ops;

impl<const T: usize> ops::Not for BitBoard<T> {
    type Output = BitBoard<T>;

    fn not(self) -> Self::Output {
        if T == 1 {
            return BitBoard {
                bits: [!self.bits[0]; T],
            };
        }

        BitBoard::<T> {
            bits: self
                .bits
                .iter()
                .map(|el| !el)
                .collect::<Vec<_>>()
                .try_into()
                .expect(&format!(
                    "Could not convert BitBoard data vector into an array during unary operation."
                )),
        }
    }
}

impl<const T: usize> ops::BitAndAssign<BitBoard<T>> for BitBoard<T> {
    fn bitand_assign(&mut self, rhs: BitBoard<T>) {
        self.effect(rhs, |el| el.0 & el.1)
    }
}

impl<const T: usize> ops::BitAnd<BitBoard<T>> for BitBoard<T> {
    type Output = BitBoard<T>;

    fn bitand(self, rhs: BitBoard<T>) -> Self::Output {
        self.apply(rhs, |el| el.0 & el.1)
    }
}

impl<const T: usize> ops::BitOr<BitBoard<T>> for BitBoard<T> {
    type Output = BitBoard<T>;

    fn bitor(self, rhs: BitBoard<T>) -> Self::Output {
        self.apply(rhs, |el| el.0 | el.1)
    }
}

impl<const T: usize> ops::BitOrAssign<BitBoard<T>> for BitBoard<T> {
    fn bitor_assign(&mut self, rhs: BitBoard<T>) {
        self.effect(rhs, |el| el.0 | el.1)
    }
}

impl<const T: usize> ops::BitXor<BitBoard<T>> for BitBoard<T> {
    type Output = BitBoard<T>;

    fn bitxor(self, rhs: BitBoard<T>) -> Self::Output {
        self.apply(rhs, |el| el.0 ^ el.1)
    }
}

impl<const T: usize> ops::BitXorAssign<BitBoard<T>> for BitBoard<T> {
    fn bitxor_assign(&mut self, rhs: BitBoard<T>) {
        self.effect(rhs, |el| el.0 ^ el.1)
    }
}

impl<const T: usize> ops::Shl<u32> for BitBoard<T> {
    type Output = BitBoard<T>;

    fn shl(self, rhs: u32) -> Self::Output {
        if T == 1 {
            return BitBoard {
                bits: [self.bits[0] << rhs; T],
            };
        }

        let mut bit_board = self.clone();
        bit_board <<= rhs;

        bit_board
    }
}

impl<const T: usize> ops::ShlAssign<u32> for BitBoard<T> {
    fn shl_assign(&mut self, mut rhs: u32) {
        if T == 1 {
            self.bits = [self.bits[0] << rhs; T];
            return;
        }

        while rhs > 128 {
            *self <<= 128;
            rhs -= 128;
        }

        let mask: u128 = u128::MAX - ((1 << (128 - rhs)) - 1); // Mask to get last `rhs` bits of integer (starting from LSB)
        for i in 0..T {
            let bits = (self.bits[i] & mask) >> (128 - rhs); // Apply mask and shift the bits over to be the first bits of the integer
            self.bits[i] = self.bits[i] << rhs;
            if i == 0 {
                continue;
            }

            self.bits[i - 1] |= bits;
        }
    }
}

impl<const T: usize> ops::Shr<u32> for BitBoard<T> {
    type Output = BitBoard<T>;

    fn shr(self, rhs: u32) -> Self::Output {
        if T == 1 {
            return BitBoard {
                bits: [self.bits[0] >> rhs; T],
            };
        }

        let mut bit_board = self.clone();
        bit_board >>= rhs;

        bit_board
    }
}

impl<const T: usize> ops::ShrAssign<u32> for BitBoard<T> {
    fn shr_assign(&mut self, mut rhs: u32) {
        if T == 1 {
            self.bits = [self.bits[0] >> rhs; T];
            return;
        }

        while rhs > 128 {
            *self >>= 128;
            rhs -= 128;
        }

        let mask: u128 = (1 << rhs) - 1; // Mask to get first `rhs` bits of integer (starting from LSB)
        for i in (0..T).rev() {
            let bits = (self.bits[i] & mask) << (128 - rhs); // Apply mask and shift the bits over to be the last bits of the integer
            self.bits[i] = self.bits[i] >> rhs;
            if i == T - 1 {
                continue;
            }

            self.bits[i + 1] |= bits;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BitBoard;

    #[test]
    fn not() {
        assert_eq!(
            !BitBoard::from_data([u128::MAX, u128::MAX]),
            BitBoard::from_data([0, 0])
        );
        assert_eq!(
            !BitBoard::from_data([0, 0]),
            BitBoard::from_data([u128::MAX, u128::MAX])
        );
        assert_eq!(
            !!BitBoard::from_data([4945856, 748691]),
            BitBoard::from_data([4945856, 748691])
        );
    }

    #[test]
    fn and() {
        assert_eq!(
            BitBoard::from_data([1, 0]) & BitBoard::from_data([0, 1]),
            BitBoard::from_data([0, 0])
        );
        assert_eq!(
            BitBoard::from_data([0, 1]) & BitBoard::from_data([0, 1]),
            BitBoard::from_data([0, 1])
        );
    }

    #[test]
    fn or() {
        assert_eq!(
            BitBoard::from_data([1, 0]) | BitBoard::from_data([0, 1]),
            BitBoard::from_data([1, 1])
        );
        assert_eq!(
            BitBoard::from_data([0, 1]) | BitBoard::from_data([0, 1]),
            BitBoard::from_data([0, 1])
        );
    }

    #[test]
    fn xor() {
        assert_eq!(
            BitBoard::from_data([1, 0]) ^ BitBoard::from_data([0, 1]),
            BitBoard::from_data([1, 1])
        );
        assert_eq!(
            BitBoard::from_data([0, 1]) ^ BitBoard::from_data([0, 1]),
            BitBoard::from_data([0, 0])
        );
    }

    #[test]
    fn shl() {
        assert_eq!(
            BitBoard::from_data([0, (u128::MAX >> 1) + 1]) << 1,
            BitBoard::from_data([1, 0])
        );
    }

    #[test]
    fn shr() {
        assert_eq!(
            BitBoard::from_data([1, 0]) >> 1,
            BitBoard::from_data([0, (u128::MAX >> 1) + 1])
        );
    }
}
