use crate::BitSet;
use std::ops;

impl<const T: usize> ops::Not for BitSet<T> {
    type Output = BitSet<T>;

    fn not(self) -> Self::Output {
        BitSet::<T> {
            data: self.data.iter().map(|el| !el).collect::<Vec<_>>().try_into().unwrap()
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

impl<const T: usize> ops::Shl<u128> for BitSet<T> {
    type Output = BitSet<T>;

    fn shl(self, rhs: u128) -> Self::Output {
        if T == 1 {
            return BitSet {
                data: [ self.data[0] << rhs; T ]
            };
        }
        
        let mut bitset = self.clone();
        bitset <<= rhs;

        bitset
    }
}

impl<const T: usize> ops::ShlAssign<u128> for BitSet<T> {
    fn shl_assign(&mut self, mut rhs: u128) {
        if T == 1 {
            self.data = [ self.data[0] << rhs; T ];
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

impl<const T: usize> ops::Shr<u128> for BitSet<T> {
    type Output = BitSet<T>;

    fn shr(self, rhs: u128) -> Self::Output {
        if T == 1 {
            return BitSet {
                data: [ self.data[0] >> rhs; T ]
            };
        }
        
        let mut bitset = self.clone();
        bitset >>= rhs;

        bitset
    }
}

impl<const T: usize> ops::ShrAssign<u128> for BitSet<T> {
    fn shr_assign(&mut self, mut rhs: u128) {
        if T == 1 {
            self.data = [ self.data[0] >> rhs; T ];
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