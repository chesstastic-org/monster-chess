use std::cmp::Ordering;

use super::BitBoard;

impl<const T: usize> PartialOrd<BitBoard<T>> for BitBoard<T> {
    fn partial_cmp(&self, other: &BitBoard<T>) -> Option<Ordering> {
        if T == 1 {
            return self.bits[0].partial_cmp(&other.bits[0]);
        }

        for (a, b) in self.bits.iter().zip(other.bits) {
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

    use super::BitBoard;

    #[test]
    fn ord() {
        assert_eq!(
            BitBoard::from_data([u128::MAX, 1]).cmp(&BitBoard::from_data([u128::MAX, 0])),
            Ordering::Greater
        );

        assert_eq!(
            BitBoard::from_data([u128::MAX, 0]).cmp(&BitBoard::from_data([u128::MAX, 1])),
            Ordering::Less
        );

        assert_eq!(
            BitBoard::from_data([1, u128::MAX]).cmp(&BitBoard::from_data([0, u128::MAX])),
            Ordering::Greater
        );

        assert_eq!(
            BitBoard::from_data([0, u128::MAX]).cmp(&BitBoard::from_data([1, u128::MAX])),
            Ordering::Less
        );

        assert_eq!(
            BitBoard::from_data([4, 3]).cmp(&BitBoard::from_data([4, 3])),
            Ordering::Equal
        );
    }
}
