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