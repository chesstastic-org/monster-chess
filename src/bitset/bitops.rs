use crate::BitSet;
use std::ops;

impl<const T: usize> ops::Not for BitSet<T> {
    type Output = BitSet<T>;

    fn not(self) -> Self::Output {
        BitSet::<T> {
            data: self
                .data
                .iter()
                .map(|el| !el)
                .collect::<Vec<_>>()
                .try_into()
                .expect(&format!(
                    "Could not convert BitSet data vector into an array during unary operation."
                )),
        }
    }
}

impl<const T: usize> ops::BitAndAssign<&BitSet<T>> for BitSet<T> {
    fn bitand_assign(&mut self, rhs: &BitSet<T>) {
        self.effect(rhs, |el| el.0 & el.1)
    }
}

impl<const T: usize> ops::BitAnd<&BitSet<T>> for BitSet<T> {
    type Output = BitSet<T>;

    fn bitand(self, rhs: &BitSet<T>) -> Self::Output {
        self.apply(rhs, |el| el.0 & el.1)
    }
}

impl<const T: usize> ops::BitOr<&BitSet<T>> for BitSet<T> {
    type Output = BitSet<T>;

    fn bitor(self, rhs: &BitSet<T>) -> Self::Output {
        self.apply(rhs, |el| el.0 | el.1)
    }
}

impl<const T: usize> ops::BitOrAssign<&BitSet<T>> for BitSet<T> {
    fn bitor_assign(&mut self, rhs: &BitSet<T>) {
        self.effect(rhs, |el| el.0 | el.1)
    }
}

impl<const T: usize> ops::BitXor<&BitSet<T>> for BitSet<T> {
    type Output = BitSet<T>;

    fn bitxor(self, rhs: &BitSet<T>) -> Self::Output {
        self.apply(rhs, |el| el.0 ^ el.1)
    }
}

impl<const T: usize> ops::BitXorAssign<&BitSet<T>> for BitSet<T> {
    fn bitxor_assign(&mut self, rhs: &BitSet<T>) {
        self.effect(rhs, |el| el.0 ^ el.1)
    }
}

impl<const T: usize> ops::Shl<u32> for BitSet<T> {
    type Output = BitSet<T>;

    fn shl(self, rhs: u32) -> Self::Output {
        if T == 1 {
            return BitSet {
                data: [self.data[0] << rhs; T],
            };
        }

        let mut bitset = self.clone();
        bitset <<= rhs;

        bitset
    }
}

impl<const T: usize> ops::ShlAssign<u32> for BitSet<T> {
    fn shl_assign(&mut self, mut rhs: u32) {
        if T == 1 {
            self.data = [self.data[0] << rhs; T];
            return;
        }

        while rhs > 128 {
            *self <<= 128;
            rhs -= 128;
        }

        let mask: u128 = u128::MAX - ((1 << (128 - rhs)) - 1); // Mask to get last `rhs` bits of integer (starting from LSB)
        for i in 0..T {
            let bits = (self.data[i] & mask) >> (128 - rhs); // Apply mask and shift the bits over to be the first bits of the integer
            self.data[i] = self.data[i] << rhs;
            if i == 0 {
                continue;
            }

            self.data[i - 1] |= bits;
        }
    }
}

impl<const T: usize> ops::Shr<u32> for BitSet<T> {
    type Output = BitSet<T>;

    fn shr(self, rhs: u32) -> Self::Output {
        if T == 1 {
            return BitSet {
                data: [self.data[0] >> rhs; T],
            };
        }

        let mut bitset = self.clone();
        bitset >>= rhs;

        bitset
    }
}

impl<const T: usize> ops::ShrAssign<u32> for BitSet<T> {
    fn shr_assign(&mut self, mut rhs: u32) {
        if T == 1 {
            self.data = [self.data[0] >> rhs; T];
            return;
        }

        while rhs > 128 {
            *self >>= 128;
            rhs -= 128;
        }

        let mask: u128 = (1 << rhs) - 1; // Mask to get first `rhs` bits of integer (starting from LSB)
        for i in (0..T).rev() {
            let bits = (self.data[i] & mask) << (128 - rhs); // Apply mask and shift the bits over to be the last bits of the integer
            self.data[i] = self.data[i] >> rhs;
            if i == T - 1 {
                continue;
            }

            self.data[i + 1] |= bits;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::BitSet;

    #[test]
    fn not() {
        assert_eq!(
            !BitSet::from_data([u128::MAX, u128::MAX]),
            BitSet::from_data([0, 0])
        );
        assert_eq!(
            !BitSet::from_data([0, 0]),
            BitSet::from_data([u128::MAX, u128::MAX])
        );
        assert_eq!(
            !!BitSet::from_data([4945856, 748691]),
            BitSet::from_data([4945856, 748691])
        );
    }

    #[test]
    fn and() {
        assert_eq!(
            BitSet::from_data([1, 0]) & &BitSet::from_data([0, 1]),
            BitSet::from_data([0, 0])
        );
        assert_eq!(
            BitSet::from_data([0, 1]) & &BitSet::from_data([0, 1]),
            BitSet::from_data([0, 1])
        );
    }

    #[test]
    fn or() {
        assert_eq!(
            BitSet::from_data([1, 0]) | &BitSet::from_data([0, 1]),
            BitSet::from_data([1, 1])
        );
        assert_eq!(
            BitSet::from_data([0, 1]) | &BitSet::from_data([0, 1]),
            BitSet::from_data([0, 1])
        );
    }

    #[test]
    fn xor() {
        assert_eq!(
            BitSet::from_data([1, 0]) ^ &BitSet::from_data([0, 1]),
            BitSet::from_data([1, 1])
        );
        assert_eq!(
            BitSet::from_data([0, 1]) ^ &BitSet::from_data([0, 1]),
            BitSet::from_data([0, 0])
        );
    }

    #[test]
    fn shl() {
        assert_eq!(
            BitSet::from_data([0, (u128::MAX >> 1) + 1]) << 1,
            BitSet::from_data([1, 0])
        );
    }

    #[test]
    fn shr() {
        assert_eq!(
            BitSet::from_data([1, 0]) >> 1,
            BitSet::from_data([0, (u128::MAX >> 1) + 1])
        );
    }
}
