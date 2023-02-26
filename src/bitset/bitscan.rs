use crate::BitSet;

impl<const T: usize> BitSet<T> {
    pub fn bitscan_forward(&self) -> u32 {
        debug_assert!(self.is_set(), "Bitscan Reverse only works for non-empty bitsets.");

        if T == 1 {
            self.data[0].leading_zeros()
        } else {
            let mut zeros: u32 = 0;
            for i in 0..T {
                let data = self.data[i];
                if data != 0 {
                    zeros += data.leading_zeros();
                    break;
                }

                zeros += 128;
            }
            zeros
        }
    }

    pub fn bitscan_reverse(&self) -> u32 {
        debug_assert!(self.is_set(), "Bitscan Forward only works for non-empty bitsets.");

        if T == 1 {
            self.data[0].trailing_zeros()
        } else {
            let mut zeros: u32 = 0;
            for i in (0..T).rev() {
                let data = self.data[i];
                if data != 0 {
                    zeros += data.trailing_zeros();
                    break;
                }

                zeros += 128;
            }
            zeros
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{BitSet};

    #[test]
    fn bitscan_forward() {
        assert_eq!(BitSet::from_data([ 0, 1 ]).bitscan_forward(), 0);
        assert_eq!(BitSet::from_data([ (u128::MAX >> 1) + 1, 0 ]).bitscan_forward(), 255);
    }

    #[test]
    fn bitscan_reverse() {
        assert_eq!(BitSet::from_data([ 0, 1 ]).bitscan_reverse(), 255);
        assert_eq!(BitSet::from_data([ (u128::MAX >> 1) + 1, 0 ]).bitscan_reverse(), 0);
    }
}