mod bitset;
mod board;
mod pieces;

pub use bitset::*;
pub use board::*;
pub use pieces::*;

fn main() {
    let test = BitSet::<2>::from_data([ u128::MAX; 2 ]);
    for i in &(test >> &5).data {
        println!("{:#0130b}", i);
    }
}