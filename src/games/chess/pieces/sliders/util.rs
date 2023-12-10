use crate::{board::{
    AttackLookup, Board,
}, bitboard::BitBoard};

pub fn get_moves_ray<const T: usize>(
    mut from: BitBoard<T>,
    slider: impl Fn(BitBoard<T>) -> BitBoard<T>,
    can_stop: impl Fn(BitBoard<T>) -> bool,
) -> BitBoard<T> {
    let mut moves = BitBoard::new();
    if can_stop(from) {
        return moves;
    }

    loop {
        from = slider(from);
        moves |= from;

        if can_stop(from) {
            break;
        }
    }

    moves
}

pub fn get_ray_attacks<const T: usize>(
    board: &Board<T>,
    from: BitBoard<T>,
    from_bit: usize,
    dir: u32,
    ray_attacks: &AttackLookup<T>,
) -> BitBoard<T> {
    let dir_usize = dir as usize;
    let mut attacks = ray_attacks[from_bit][dir_usize];
    let blocker = attacks & board.state.all_pieces;
    if blocker.is_set() {
        let square = if from < blocker {
            blocker.bitscan_forward()
        } else {
            blocker.bitscan_reverse()
        };

        attacks ^= ray_attacks[square as usize][dir_usize];
    }
    return attacks;
}
