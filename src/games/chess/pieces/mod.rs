mod king;
mod knight;
mod pawn;
mod sliders;

pub const PAWN: usize = 0;
pub const KNIGHT: usize = 1;
pub const BISHOP: usize = 2;
pub const ROOK: usize = 3;
pub const QUEEN: usize = 4;
pub const KING: usize = 5;

pub use king::*;
pub use knight::*;
pub use pawn::*;
pub use sliders::*;
