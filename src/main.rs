use monster_chess::bitboard::{generate_ranks, generate_files, BitBoard};



fn main() {
    let board = BitBoard::<1>::from_element(0x20140c0c142000);

    println!("{}", board.display(8, 8));
    println!("-");
    println!("{}", board.flip_horizontally(generate_files(8, 8), 8).display(8, 8));
}