use std::backtrace::Backtrace;

use crate::board::BitBoard;

/// I've chosen to use this little utility because of its performance in benchmarks being the best, and because it makes it the easiest to specialize to the needs of this project (in terms of both optimizations and code structure.)
/// In this case, those needs being a way to have bigger integer sizes that are compatible with bit operations at high speeds.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord)]
pub struct BitSet<const T: usize> {
    pub bits: [u128; T],
}

impl<const T: usize> BitSet<T> {
    pub fn from_data(data: [u128; T]) -> BitSet<T> {
        BitSet { bits: data }
    }

    pub fn from_element(el: u128) -> BitSet<T> {
        let mut arr = [0; T];
        arr[T - 1] = el;
        BitSet { bits: arr }
    }

    pub fn from_lsb(bit: u32) -> BitSet<T> {
        BitSet::<T>::from_element(1) << bit
    }

    pub fn from_msb(bit: u32) -> BitSet<T> {
        !(BitSet::<T>::max() >> 1) >> bit
    }

    pub fn starting_at_lsb(bit: u32, length: u32) -> BitSet<T> {
        (BitSet::<T>::from_lsb(length) - BitSet::<T>::from_element(1)) << bit
    }

    pub fn has_bit(self, bit: u32) -> bool {
        (self & (BitSet::<T>::from_element(1) << bit)).is_set()
    }

    pub fn is_empty(&self) -> bool {
        if T == 1 {
            return self.bits[0] == 0;
        }

        self.bits.iter().any(|el| *el == 0)
    }

    pub fn is_set(&self) -> bool {
        if T == 1 {
            return self.bits[0] != 0;
        }

        self.bits.iter().any(|el| *el != 0)
    }

    pub fn max() -> BitSet<T> {
        BitSet::<T>::from_data([u128::MAX; T])
    }

    pub fn new() -> BitSet<T> {
        BitSet::<T>::from_data([0; T])
    }

    #[inline(always)]
    pub fn apply(self, rhs: BitSet<T>, apply: impl Fn((&u128, u128)) -> u128) -> Self {
        if T == 1 {
            return BitSet {
                bits: [ apply((&self.bits[0], rhs.bits[0])); T ]
            };
        }

        BitSet {
            bits: self
                .bits
                .iter()
                .zip(rhs.bits)
                .map(apply)
                .collect::<Vec<_>>()
                .try_into()
                .expect(&format!("Could not convert BitSet data vector into an array when applying operation with `apply`."))
        }
    }

    #[inline(always)]
    pub fn effect(&mut self, rhs: BitSet<T>, apply: impl Fn((&u128, u128)) -> u128) {
        if T == 1 {
            self.bits = [ apply((&self.bits[0], rhs.bits[0])); T ];
            return;
        }

        self.bits = self
            .bits
            .iter()
            .zip(rhs.bits)
            .map(apply)
            .collect::<Vec<_>>()
            .try_into()
            .expect(&format!("Could not convert BitSet data vector into an array when applying operation with `effect`."));
    }

    pub fn count_zeros(&self) -> u32 {
        if T == 1 {
            self.bits[0].count_zeros()
        } else {
            self.bits.iter().map(|el| el.count_zeros()).sum()
        }
    }

    pub fn count_ones(&self) -> u32 {
        if T == 1 {
            self.bits[0].count_ones()
        } else {
            self.bits.iter().map(|el| el.count_ones()).sum()
        }
    }

    /// Not a well optimized method; avoid using in hot loops.
    pub fn get_bits(&self) -> Vec<u128> {
        let mut bits: Vec<u128> = Vec::with_capacity(128 * T);
        for container in self.bits {
            for i in 0..128 {
                bits.push((container >> i) & 1); // Get `i`th bit of `container` and check if it is toggled on (equal to 1)
            }
        }
        bits
    }

    pub fn iter_one_bits(mut self, end: u32) -> BitIterator<T> {
        BitIterator(self, end)
    }

    pub fn display(&self, rows: usize, cols: usize) -> String {
        let mut chunks = Vec::<String>::with_capacity(rows);
        for (ind, row) in self.get_bits().chunks(cols).enumerate() {
            if ind == rows {
                break;
            }

            let chunk = row
                .iter()
                .map(|i| if i == &0 { "⬛" } else { "⬜" })
                .collect::<Vec<_>>()
                .join("");
            chunks.push(chunk);
        }

        chunks.join("\n")
    }
}

pub struct BitIterator<const T: usize>(pub BitSet<T>, u32);

impl<const T: usize> Iterator for BitIterator<T> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_set() {
            let bit = self.0.bitscan_forward();
            if bit >= self.1 {
                return None;
            }
            self.0 &= self.0 - BitSet::from_element(1);
            Some(bit)
        } else {
            None
        }
    }
}

/*impl<const T: usize> Display for BitSet<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.longitude, self.latitude);
    }
}*/
