/// `BitBoard<T>` is essentially a wrapper around a `[u128; T]`.
/// It has multiple `u128`s so it can serve as essentially a bigger unsigned integer for `T > 1`.
/// It handles bit operations and even some mathematical operations to operate essentially as if it was any other integer type.
/// It also has methods specifically related to its use as a BitBoard, like moving it up, down, right, left, etc.
/// For `T = 1` (a BitBoard with only one `u128`), it should compile down to esssentially a single `u128`, and should be very fast.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord)]
pub struct BitBoard<const T: usize> {
    pub bits: [u128; T],
}

impl<const T: usize> BitBoard<T> {
    /// Create a `BitBoard` from a backing array.
    pub fn from_data(data: [u128; T]) -> BitBoard<T> {
        BitBoard { bits: data }
    }

    /// Create a `BitBoard` from a single `u128`.
    pub fn from_element(el: u128) -> BitBoard<T> {
        let mut arr = [0; T];
        arr[T - 1] = el;
        BitBoard { bits: arr }
    }

    /// Create a `BitBoard` with a single bit from the index of that bit from the LSB.
    pub fn from_lsb(bit: u16) -> BitBoard<T> {
        BitBoard::<T>::from_element(1) << bit
    }

    /// Create a `BitBoard` with a single bit from the index of that bit from the MSB.
    pub fn from_msb(bit: u16) -> BitBoard<T> {
        !(BitBoard::<T>::max() >> 1) >> bit
    }

    /// Create a `BitBoard` starting at a given bit, and continuing for a given length.
    pub fn starting_at_lsb(bit: u16, length: u16) -> BitBoard<T> {
        (BitBoard::<T>::from_lsb(length) - BitBoard::<T>::from_element(1)) << bit
    }

    /// Check if a `BitBoard` has a given bit.
    pub fn has_bit(self, bit: u16) -> bool {
        (self & (BitBoard::<T>::from_element(1) << bit)).is_set()
    }

    /// Check if a `BitBoard` is empty (if that `BitBoard` is `0`.)
    pub fn is_empty(&self) -> bool {
        if T == 1 {
            return self.bits[0] == 0;
        }

        self.bits.iter().all(|el| *el == 0)
    }

    /// Check if a `BitBoard` is set (if that `BitBoard` isn't `0`, if it has at least one set bit.)
    pub fn is_set(&self) -> bool {
        if T == 1 {
            return self.bits[0] != 0;
        }

        self.bits.iter().any(|el| *el != 0)
    }

    /// Get the highest possible value for a given `BitBoard`.
    pub fn max() -> BitBoard<T> {
        BitBoard::<T>::from_data([u128::MAX; T])
    }

    /// Initialize a new, empty `BitBoard`.
    pub fn new() -> BitBoard<T> {
        BitBoard::<T>::from_data([0; T])
    }

    /// A utility method primarily for internally handling bit operations.
    /// Apply a function to each of the `u128`s of two `BitBoards`.
    /// For instance, `bitboard.apply(rhs, |el| el.0 | el.1)` would get a `BitBoard` for the `or` bit operation.
    #[inline(always)]
    pub fn apply(self, rhs: BitBoard<T>, apply: impl Fn((&u128, u128)) -> u128) -> Self {
        if T == 1 {
            return BitBoard {
                bits: [apply((&self.bits[0], rhs.bits[0])); T],
            };
        }

        BitBoard {
            bits: self
                .bits
                .iter()
                .zip(rhs.bits)
                .map(apply)
                .collect::<Vec<_>>()
                .try_into()
                .expect(&format!("Could not convert BitBoard data vector into an array when applying operation with `apply`."))
        }
    }

    /// A utility method primarily for internally handling bit operations.
    /// Apply a function to each of the `u128`s of two `BitBoard`s, mutating the first of those `BitBoard`s.
    /// For instance, `bitboard.effect(rhs, |el| el.0 | el.1)` would apply the `or` operation to this `BitBoard`.
    #[inline(always)]
    pub fn effect(&mut self, rhs: BitBoard<T>, apply: impl Fn((&u128, u128)) -> u128) {
        if T == 1 {
            self.bits = [apply((&self.bits[0], rhs.bits[0])); T];
            return;
        }

        self.bits = self
            .bits
            .iter()
            .zip(rhs.bits)
            .map(apply)
            .collect::<Vec<_>>()
            .try_into()
            .expect(&format!("Could not convert BitBoard data vector into an array when applying operation with `effect`."));
    }

    /// Count the number of zero bits in a given `BitBoard`.
    pub fn count_zeros(&self) -> u32 {
        if T == 1 {
            self.bits[0].count_zeros()
        } else {
            self.bits.iter().map(|el| el.count_zeros()).sum()
        }
    }

    /// Count the number of one bits in a given `BitBoard`.
    pub fn count_ones(&self) -> u32 {
        if T == 1 {
            self.bits[0].count_ones()
        } else {
            self.bits.iter().map(|el| el.count_ones()).sum()
        }
    }

    /// Outputs an array of all bits, with `0` if they're off, and `1` otherwise.
    /// Not a well optimized method; avoid using in hot loops.
    pub fn get_bits(&self) -> Vec<u128> {
        let mut bits: Vec<u128> = Vec::with_capacity(128 * T);
        for container in self.bits.iter().rev() {
            for i in 0..128 {
                bits.push((container >> i) & 1); // Get `i`th bit of `container` and check if it is toggled on (equal to 1)
            }
        }
        bits
    }

    /// Iterates over all set bits in the `BitBoard`.
    pub fn iter_set_bits(mut self, end: u16) -> BitIterator<T> {
        BitIterator(self, end)
    }

    /// Displays a `BitBoard`, showing ⬛ if the bit is off, and ⬜ if it's on.
    /// Consider using this if you need to debug a `BitBoard`.
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

/// `BitIterator` is used to efficiently iterate over all of the bits in a `BitBoard` with `BitBoard.iter_set_bits()`.
pub struct BitIterator<const T: usize>(pub BitBoard<T>, u16);

impl<const T: usize> Iterator for BitIterator<T> {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_set() {
            let bit = self.0.bitscan_forward();
            if bit >= self.1 {
                return None;
            }
            self.0 &= self.0 - BitBoard::from_element(1);
            Some(bit)
        } else {
            None
        }
    }
}
