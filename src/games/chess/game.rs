use crate::{
    bitboard::{Direction, BitBoard},
    board::{
        actions::{Action, HistoryMove, HistoryState, IndexedPreviousBoard, PreviousBoard},
        edges::Edges,
        fen::{
            FenArgument, FenDecodeError, FenFullMoves, FenOptions, FenState, FenSubMoves,
            FenTeamArgument, PostProcess,
        },
        game::{Game, MoveController, DefaultZobristController},
        pieces::{Piece, PieceSymbol},
        AttackDirections, Board, Cols, PieceType, zobrist::ZobristHashTable,
    },
};

use super::{pieces::{
    down, up, BishopPiece, KingPiece, KnightPiece, PawnPiece, QueenPiece, RookPiece,
}, ChessMoveController, ChessPostProcess, ChessCastlingRights, ChessEnPassant, ChessResolution};

pub const ATTACKS_MODE: u32 = 1;

const PAWN: &dyn Piece<1> = &PawnPiece;
const KNIGHT: &dyn Piece<1> = &KnightPiece;
const BISHOP: &dyn Piece<1> = &BishopPiece;
const ROOK: &dyn Piece<1> = &RookPiece;
const QUEEN: &dyn Piece<1> = &QueenPiece;
const KING: &dyn Piece<1> = &KingPiece;

pub struct Chess;

impl Chess {
    pub fn create() -> Game<1> {
        Game {
            teams: 2,
            turns: 1,
            rows: 8,
            cols: 8,
            squares: 64,
            zobrist_controller: Box::new(DefaultZobristController),
            zobrist: ZobristHashTable::<1>::generate(64, 2, 6, 65, || fastrand::u64(0..u64::MAX)),
            name: String::from("Chess"),
            pieces: vec![PAWN, KNIGHT, BISHOP, ROOK, QUEEN, KING],
            controller: Box::new(ChessMoveController),
            resolution: Box::new(ChessResolution),
            fen_options: FenOptions {
                state: FenState { first_moves: false, gaps: '-' },
                args: vec![
                    (
                        "team to move".to_string(),
                        Box::new(FenTeamArgument::Teams(vec!['w', 'b'])),
                    ),
                    ("castling rights".to_string(), Box::new(ChessCastlingRights)),
                    ("en passant".to_string(), Box::new(ChessEnPassant)),
                    ("half moves".to_string(), Box::new(FenSubMoves)),
                    ("full moves".to_string(), Box::new(FenFullMoves)),
                ],
                post_process: Box::new(ChessPostProcess),
                default_fen: String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
            }
        }
    }
}
