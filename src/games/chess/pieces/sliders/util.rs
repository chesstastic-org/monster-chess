use crate::board::{pieces::{PieceSymbol, Piece}, PieceType, BitBoard, Board, AttackDirections, AttackLookup};

pub fn get_moves_ray(
    mut from: BitBoard,
    slider: impl Fn(BitBoard) -> BitBoard,
    can_stop: impl Fn(BitBoard) -> bool,
) -> BitBoard {
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

pub fn get_ray_attacks(
    board: &Board,
    from: BitBoard,
    from_bit: usize,
    dir: u32,
    ray_attacks: &AttackLookup,
) -> BitBoard {
    let dir_usize = dir as usize;
    let mut attacks = ray_attacks[from_bit][dir_usize];
    let mut blocker = attacks;
    blocker &= board.state.all_pieces;
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
