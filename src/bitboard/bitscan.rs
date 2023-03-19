use super::BitBoard;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    LEFT,
    RIGHT,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::RIGHT => Direction::LEFT,
            Direction::LEFT => Direction::LEFT
        }
    }
}

impl<const T: usize> BitBoard<T> {
    /// A forward bitscan, which finds the least significant 1-bit.
    pub fn bitscan_forward(&self) -> u16 {
        assert!(
            self.is_set(),
            "Bitscan Forward only works for non-empty BitBoards."
        );

        if T == 1 {
            self.bits[0].trailing_zeros() as u16
        } else {
            let mut zeros: u16 = 0;
            for i in (0..T).rev() {
                let data = self.bits[i];
                if data != 0 {
                    zeros += data.trailing_zeros() as u16;
                    break;
                }

                zeros += 128;
            }
            zeros
        }
    }

    /// A reverse bitscan, which finds the most significant 1-bit.
    pub fn bitscan_reverse(&self) -> u16 {
        assert!(
            self.is_set(),
            "Bitscan Reverse only works for non-empty BitBoards."
        );

        if T == 1 {
            127 - self.bits[0].leading_zeros() as u16
        } else {
            let mut zeros: u16 = 0;
            for i in 0..T {
                let data = self.bits[i];
                if data != 0 {
                    zeros += data.leading_zeros() as u16;
                    break;
                }

                zeros += 128;
            }
            (((T as u16) * 128) - 1) - zeros
        }
    }

    pub fn bitscan(&self, direction: Direction) -> u16 {
        match direction {
            Direction::LEFT => self.bitscan_forward(),
            Direction::RIGHT => self.bitscan_reverse(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BitBoard;

    #[test]
    fn bitscan_forward() {
        assert_eq!(BitBoard::from_data([0, 1]).bitscan_forward(), 0);
        assert_eq!(BitBoard::from_data([1, 1]).bitscan_forward(), 0);
        assert_eq!(BitBoard::from_data([3, 3]).bitscan_forward(), 0);
        assert_eq!(BitBoard::from_data([1, 0]).bitscan_forward(), 128);
    }

    #[test]
    fn bitscan_reverse() {
        assert_eq!(BitBoard::from_data([0, 1]).bitscan_reverse(), 0);
        assert_eq!(BitBoard::from_data([1, 1]).bitscan_reverse(), 128);
        assert_eq!(BitBoard::from_data([3, 3]).bitscan_reverse(), 129);
        assert_eq!(BitBoard::from_data([1, 0]).bitscan_reverse(), 128);
    }
}
