use crate::{AttackLookup, BitBoard, Board};

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
        moves |= &from;

        if can_stop(from) {
            break;
        }
    }

    moves
}

pub fn get_ray_attacks(
    board: &Board,
    from: BitBoard,
    dir: u32,
    ray_attacks: &AttackLookup,
) -> BitBoard {
    let mut attacks = ray_attacks[from.bitscan_forward() as usize as usize][dir as usize];
    let blocker = attacks & &board.state.all_pieces;
    if blocker.is_set() {
        let square = if from >= blocker {
            blocker.bitscan_reverse()
        } else {
            blocker.bitscan_forward()
        };
        
        attacks ^= &ray_attacks[square as usize][dir as usize];
    }
    return attacks;
}
