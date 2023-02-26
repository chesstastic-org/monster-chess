<div align = "center">
<h1>monster-chess</h1>
</div>

# Overview

`monster-chess` is a fairy chess move generation library written in Rust. It was created as part of the Chesstastic project (which aims to allow for users to easily create chess variants, play against others in those chess variants and analyze games of said variants), and is meant to handle move generation and move validation logic that happens in those variants. Currently, the library isn't functional, but hopefully it will be in the near future.

## Note

If you are only aiming to support chess or chess960, you should use the [cozy-chess](https://github.com/analog-hors/cozy-chess/) library, which will have much better performance and better code quality then the `monster-chess` library.

## Implementation

`monster-chess` uses a general implementation of [Bitboards](https://www.chessprogramming.org/Bitboards) to extend to larger board sizes, using a custom made `BitSet` data type.

```rust
pub struct BitSet<const T : usize> {
    pub data: [ u128; T ]
}
```

BitSets are designed in such a way that if the BitSet only has to store one `u128`, it would (ideally) be optimized to be as fast as it would natively. Bitsets support all **bitwise** operators, alongside addition, subtraction, and two custom methods for [bitscans](https://www.chessprogramming.org/BitScan).

The library generates movement for pieces by taking in a bitboard with one toggled bit representing where the piece currently is, and applying bitwise operations to it. In native chess, there are three main types of pieces.

- **Delta Pieces**, like the Knight and King, which move in the _same way_ every time, no matter which square they are on (except out of bounds squares.)
- **Slider Pieces**, like the Bishop, Rook, and Queen, which move almost in the same way every time looking in a certain direction, but can be blocked by pieces.
- **Pawns**.

To optimize move generation for Delta Pieces and Slider Pieces, `monster-chess` provides an attack lookup table. Once a board is initialized, if a piece chooses to enable attack table lookups for speeding up move generation, its moves will be generated for every possible square it can go to. This means for kings and knights, move generation is effectively instant, only requiring an attack table lookup. For rooks, bishops, and queens, the attack table is used to retrieve the directions they can move in from any given square, but additional logic is needed to block out any pieces in the way.

Attack table lookups are stored as an `AttackDirections` once retrieved, which is an alias for `Vec<BitBoard>`. This means for delta pieces, they can just store their moves in the first slot, but for sliding pieces, they can store a bitboard for each direction they can go in.

Here's an example of what the King piece would look like with `monster-chess`'s piece system.

```rust
impl Piece for KingPiece {
    // generate_moves will generate moves for attack tables
    fn generate_moves(&self, board: &Board, mut from: BitBoard) -> AttackDirections {
        let moves = ...;
        vec![ moves ]
    }   

    // can_lookup tells us if we should generate an attack table beforehand
    fn can_lookup(&self) -> bool {
        true
    }

    // get_moves will fetch the moves in the game
    fn get_moves(&self, board: &Board, from: BitBoard) -> BitBoard {
        let lookup = self.get_attack_lookup(board, from);
        match lookup {
            Some(lookup) => lookup[0],
            None => self.generate_moves(board, from)[0]
        }
    }
}
```