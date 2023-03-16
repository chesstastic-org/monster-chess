use crate::{
    bitboard::{Direction, BitBoard},
    board::{
        actions::{Action, HistoryMove, HistoryState, IndexedPreviousBoard, PreviousBoard},
        edges::Edges,
        pieces::{Piece, PieceSymbol},
        AttackDirections, Board, Cols, PieceType,
    },
    games::chess::game::ATTACKS_MODE,
};

#[derive(Debug)] pub struct KnightPiece<const T: usize>;

fn up2_right<const T: usize>(b: BitBoard<T>, cols: Cols, edges: &Edges<T>, deep_edges: &Edges<T>) -> BitBoard<T> {
    (b & !deep_edges.top & !edges.right).up(2, cols).right(1)
}

fn up_right2<const T: usize>(b: BitBoard<T>, cols: Cols, edges: &Edges<T>, deep_edges: &Edges<T>) -> BitBoard<T> {
    (b & !edges.top & !deep_edges.right).up(1, cols).right(2)
}

fn down2_right<const T: usize>(b: BitBoard<T>, cols: Cols, edges: &Edges<T>, deep_edges: &Edges<T>) -> BitBoard<T> {
    (b & !deep_edges.bottom & !edges.right)
        .down(2, cols)
        .right(1)
}

fn down_right2<const T: usize>(b: BitBoard<T>, cols: Cols, edges: &Edges<T>, deep_edges: &Edges<T>) -> BitBoard<T> {
    (b & !edges.bottom & !deep_edges.right)
        .down(1, cols)
        .right(2)
}

fn up2_left<const T: usize>(b: BitBoard<T>, cols: Cols, edges: &Edges<T>, deep_edges: &Edges<T>) -> BitBoard<T> {
    (b & !deep_edges.top & !edges.left).up(2, cols).left(1)
}

fn up_left2<const T: usize>(b: BitBoard<T>, cols: Cols, edges: &Edges<T>, deep_edges: &Edges<T>) -> BitBoard<T> {
    (b & !edges.top & !deep_edges.left).up(1, cols).left(2)
}

fn down2_left<const T: usize>(b: BitBoard<T>, cols: Cols, edges: &Edges<T>, deep_edges: &Edges<T>) -> BitBoard<T> {
    (b & !deep_edges.bottom & !edges.left).down(2, cols).left(1)
}

fn down_left2<const T: usize>(b: BitBoard<T>, cols: Cols, edges: &Edges<T>, deep_edges: &Edges<T>) -> BitBoard<T> {
    (b & !edges.bottom & !deep_edges.left).down(1, cols).left(2)
}

impl<const T: usize> Piece<T> for KnightPiece<T> {
    fn get_piece_symbol(&self) -> PieceSymbol {
        PieceSymbol::Char('n')
    }

    fn generate_lookup_moves(&self, board: &Board<T>, from: BitBoard<T>) -> AttackDirections<T> {
        let cols = board.state.cols;
        let edges = &board.state.edges[0];
        let deep_edges = &board.state.edges[1];

        let mut moves = up2_right(from, cols, edges, deep_edges);
        moves |= up_right2(from, cols, edges, deep_edges);
        moves |= down2_right(from, cols, edges, deep_edges);
        moves |= down_right2(from, cols, edges, deep_edges);
        moves |= up2_left(from, cols, edges, deep_edges);
        moves |= up_left2(from, cols, edges, deep_edges);
        moves |= down2_left(from, cols, edges, deep_edges);
        moves |= down_left2(from, cols, edges, deep_edges);

        vec![moves]
    }

    fn can_lookup(&self) -> bool {
        true
    }

    fn can_move_mask(
        &self,
        board: &Board<T>,
        from: BitBoard<T>,
        from_bit: u32,
        piece_type: usize,
        team: u32,
        mode: u32,
        to: BitBoard<T>,
    ) -> BitBoard<T> {
        self.get_attack_lookup(board, piece_type).expect("Could not find move lookup table for Knight")[from_bit as usize][0]
    }

    #[allow(unused_variables)]
    fn get_moves(
        &self,
        board: &Board<T>,
        from: BitBoard<T>,
        piece_type: usize,
        team: u32,
        mode: u32,
    ) -> BitBoard<T> {
        let lookup = self.get_attack_lookup(board, piece_type);
        match lookup {
            Some(lookup) => lookup[from.bitscan_reverse() as usize][0],
            None => self.generate_lookup_moves(board, from)[0],
        }
    }
}
