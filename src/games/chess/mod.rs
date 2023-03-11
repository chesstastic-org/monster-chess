mod game;
mod pieces;

pub use game::*;
pub use pieces::*;

#[cfg(test)]
mod tests {
    use crate::{BitSet, Board, Chess};

    #[test]
    fn startpos() {
        let mut board = Board::new(
            Chess::create(),
            2,
            (8, 8),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        );

        board.assert_perfts([20, 400, 8902, 197281, 4865609]);
    }

    #[test]
    fn castling() {
        let mut board = Board::new(
            Chess::create(),
            2,
            (8, 8),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/R3KBNR w kqKQ - 0 1",
        );

        board.assert_perfts([23, 460, 10664, 236936]);
    }

    #[test]
    fn white_promotion() {
        let mut board = Board::new(Chess::create(), 2, (8, 8), "8/5P2/8/8/8/7K/8/n6k w - - 0 1");

        board.assert_perfts([7, 25, 299, 1931]);
    }

    #[test]
    fn black_promotion() {
        let mut board = Board::new(Chess::create(), 2, (8, 8), "N6K/8/8/7k/8/8/5p2/8 b - - 0 1");

        board.assert_perfts([9, 41, 524, 3674]);
    }

    #[test]
    fn en_passant() {
        let mut board = Board::new(
            Chess::create(),
            2,
            (8, 8),
            "rnbqkbnr/1pp1pppp/8/p2pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 3",
        );

        board.assert_perfts([31, 839, 25956, 723699]);
    }

    #[test]
    fn kiwipete() {
        let mut board = Board::new(
            Chess::create(),
            2,
            (8, 8),
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
        );

        board.assert_perfts([48, 2039, 97862, 4085603]);
    }

    #[test]
    fn position_three() {
        let mut board = Board::new(
            Chess::create(),
            2,
            (8, 8),
            "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1",
        );

        board.assert_perfts([14, 191, 2812, 43238, 674624]);
    }

    #[test]
    fn position_four() {
        let mut board = Board::new(
            Chess::create(),
            2,
            (8, 8),
            "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
        );

        board.assert_perfts([6, 264, 9467, 422333]);
    }

    #[test]
    fn position_five() {
        let mut board = Board::new(
            Chess::create(),
            2,
            (8, 8),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
        );

        board.assert_perfts([44, 1486, 62379, 2103487]);
    }

    #[test]
    fn position_six() {
        let mut board = Board::new(
            Chess::create(),
            2,
            (8, 8),
            "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
        );

        board.assert_perfts([46, 2079, 89890, 3894594]);
    }

    #[test]
    fn multiple_pawns_promoting() {
        let mut board = Board::new(
            Chess::create(),
            2,
            (8, 8),
            "n1n5/PPPk4/8/8/8/8/4Kppp/5N1N b - - 0 1",
        );

        board.assert_perfts([24, 496, 9483, 182838, 3605103]);
    }
}
