use crate::{board::{fen::PostProcess, Board}};

#[derive(Debug)]
pub struct ChessPostProcess;
impl<const T: usize> PostProcess<T> for ChessPostProcess {
    fn apply(&self, board: &mut Board<T>) {
        let cols = board.state.cols;
        let edges = &board.state.edges[0];
        let mut bottom = edges.bottom;
        let mut top = edges.top;

        bottom |= bottom.up(1, cols);
        top |= top.down(1, cols);

        let first_move = (board.state.pieces[0] & board.state.teams[0] & bottom)
            | (board.state.pieces[0] & board.state.teams[1] & top)
            | (board.state.all_pieces ^ board.state.pieces[0]);
        board.state.first_move &= first_move;
    }
}