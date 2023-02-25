use std::ops;

/*
    I've chosen to use this little utility because of its performance in benchmarks being the best, and because it makes it the easiest to specialize to the needs of this project (in terms of both optimizations and code structure.)
    In this case, those needs being a way to have bigger integer sizes that are compatible with bit operations at high speeds.
*/

#[derive(Debug, Clone, Copy)]
pub struct BitSet<const T : usize> {
    pub data: [ u128; T ]
}

impl<const T: usize> BitSet<T> {
    pub fn from_data<const S: usize>(data: [ u128; S ]) -> BitSet<S> {
        BitSet {
            data
        }
    }

    pub fn from_element<const S: usize>(el: u128) -> BitSet<S> {
        let mut arr = [ el; S ];
        arr[0] = el;
        BitSet {
            data: arr
        }
    }

    pub fn new<const S: usize>() -> BitSet<S> {
        BitSet::<S>::from_data([ 0; S ])
    }

    pub fn apply(self, rhs: &BitSet<T>, apply: impl Fn((&u128, &u128)) -> u128) -> Self {
        BitSet {
            data: self.data.iter().zip(&rhs.data).map(apply).collect::<Vec<_>>().try_into().unwrap()
        }
    }

    pub fn effect(&mut self, rhs: &BitSet<T>, apply: impl Fn((&u128, &u128)) -> u128) {
        self.data = self.data.iter().zip(&rhs.data).map(apply).collect::<Vec<_>>().try_into().unwrap()
    }

    /*
        Not a well optimized method; avoid using in hot loops.
    */
    pub fn get_bits(&self) -> Vec<u128> {
        let mut bits: Vec<u128> = Vec::with_capacity(128 * T);
        for container in self.data {
            for i in 0..127 {
                bits.push((container >> i) & 1); // Get `i`th bit of `container` and check if it is toggled on (equal to 1)
            }
        }
        bits
    }
}

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
        let len = self.data.len();
        if len == 1 {
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
        let len = self.data.len();
        if len == 1 {
            self.data = [ self.data[0] << rhs; T ];
        }

        while rhs > 128 {
            *self <<= 128;
            rhs -= 128;
        }

        let mask: u128 = u128::MAX - ((1 << (128 - rhs)) - 1); // Mask to get last `rhs` bits of integer
        for i in 0..len {
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
        let len = self.data.len();
        if len == 1 {
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
        let len = self.data.len();
        if len == 1 {
            self.data = [ self.data[0] >> rhs; T ];
        }

        while rhs > 128 {
            *self >>= 128;
            rhs -= 128;
        }

        let mask: u128 = (1 << rhs) - 1; // Mask to get first `rhs` bits of integer
        for i in (0..len).rev() {
            let bits = (self.data[i] & mask) << (128 - rhs); // Apply mask and shift the bits over to be the last bits of the integer
            self.data[i] = self.data[i] >> rhs;
            if i == len - 1 {
                continue;
            }

            self.data[i + 1] |= bits;
        }
    }
}