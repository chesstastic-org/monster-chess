use std::cmp::Ordering;

use crate::BitSet;

impl<const T : usize> PartialOrd<BitSet<T>> for BitSet<T> {
    fn partial_cmp(&self, other: &BitSet<T>) -> Option<Ordering> {
        if T == 1 {
            return self.data[0].partial_cmp(&other.data[0]);
        }

        for (a, b) in self.data.iter().zip(other.data) {
            match a.partial_cmp(&b) {
                Some(Ordering::Greater) => {
                    return Some(Ordering::Greater);
                }
                Some(Ordering::Less) => {
                    return Some(Ordering::Less);
                }
                _ => {}
            }
        }

        return Some(Ordering::Equal);
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use crate::{BitSet};

    #[test]
    fn ord() {
        assert_eq!(
            BitSet::from_data([ u128::MAX, 1 ]).cmp(&BitSet::from_data([ u128::MAX, 0 ])),
            Ordering::Greater
        );

        assert_eq!(
            BitSet::from_data([ u128::MAX, 0 ]).cmp(&BitSet::from_data([ u128::MAX, 1 ])),
            Ordering::Less
        );

        assert_eq!(
            BitSet::from_data([ 1, u128::MAX ]).cmp(&BitSet::from_data([ 0, u128::MAX ])),
            Ordering::Greater
        );

        assert_eq!(
            BitSet::from_data([ 0, u128::MAX ]).cmp(&BitSet::from_data([ 1, u128::MAX ])),
            Ordering::Less
        );

        assert_eq!(
            BitSet::from_data([ 4, 3 ]).cmp(&BitSet::from_data([ 4, 3 ])),
            Ordering::Equal
        );
    }
}