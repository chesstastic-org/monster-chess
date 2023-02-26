mod board;
mod bitset;

pub use board::*;
pub use bitset::*;

fn main() {
    let mut board = Board {
        state: BoardState { 
            blockers: BitBoard::new(), 
            pieces: Vec::new(), 
            teams: Vec::new(),
            edges: generate_edge_list(8, 8),
            rows: 8, 
            cols: 8
        },
        pieces: vec![
            Box::new(KnightPiece {
                piece_type: 0
            }),
            Box::new(KingPiece {
                piece_type: 1
            })
        ],
        attack_lookup: Vec::new()
    };

    board.generate_lookups();

    let from = BitBoard::from_data([ 1 << 24 ]);
    println!("{}", board.pieces[0].get_moves(&board, from).display(8, 8));

    /*let king_pos = BitBoard::from_element(4096);
    let moves = king.generate_moves(king_pos);

    println!("{}", moves.display(8, 8));*/
}