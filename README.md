<div align = "center">
<h1>monster-chess</h1>
</div>

## Overview

`monster-chess` is a fairy chess move generation library written in Rust. It was created as part of the Chesstastic project (which aims to allow for users to easily create chess variants, play against others in those chess variants and analyze games of said variants), and is meant to handle move generation and move validation logic that happens in those variants. Currently, the library isn't functional, but hopefully it will be in the near future.

### Compatibility

When we say that we're compatible with a given game, that doesn't mean we'll necessarily be providing out-of-the-box support for it, but that the game will be implementable given the framework that `monster-chess` provides.

Types of games that we're aiming to be compatible with:
- [Chess](https://en.wikipedia.org/wiki/Chess)
- [FRC Chess](https://en.wikipedia.org/wiki/Fischer_random_chess)
- [Shogi](https://en.wikipedia.org/wiki/Shogi)
- [Maharajah and the Sepoys](https://en.wikipedia.org/wiki/Maharajah_and_the_Sepoys)

Types of games we're not aiming to be compatible with:
- [Go](https://en.wikipedia.org/wiki/Go_(game))
- [5D Chess with Multiversal Time Travel](https://store.steampowered.com/app/1349230/5D_Chess_With_Multiverse_Time_Travel/)

If you're wondering if a given game or chess variant is compatible with chess, imagine starting with the base game of chess, and see if you can do any of the following to get to your variant.

- Can I increase/decrease the board size?
- Can I remove existing pieces, or add new pieces with new types of movement?
- Can I change the capture style of pieces (eg. swapping positions with the piece instead of capturing it.)
- Can I change the movement restrictions? (eg. being allowed to move into check)
- Can I change the win conditions? (eg. being able to win by capturing all pieces.)

If so, `monster-chess` most likely will be able to be compatible with said variant.

### Note

If you are only aiming to support chess or chess960, you should use the [cozy-chess](https://github.com/analog-hors/cozy-chess/) library, which will have much better performance and better code quality then the `monster-chess` library.

## Implementation

### Bitboards

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
    // get_piece_symbol will show the symbol used for a given piece in FEN notation
    fn get_piece_symbol(&self) -> &str {
        "k"
    }

    // generate_lookup_moves will generate moves for attack tables
    fn generate_lookup_moves(&self, board: &Board, mut from: BitBoard) -> AttackDirections {
        let moves = ...;
        vec![ moves ]
    }   

    // can_lookup tells us if we should generate an attack table beforehand
    fn can_lookup(&self) -> bool {
        true
    }

    // get_moves will fetch the moves for move generation itself
    fn get_moves(&self, board: &Board, from: BitBoard) -> BitBoard {
        let lookup = self.get_attack_lookup(board, from);
        match lookup {
            Some(lookup) => lookup[from.bitscan_reverse() as usize][0],
            None => self.generate_moves(board, from)[0]
        }
    }
}
```

### FEN Representation

Because `monster-chess` is aiming to support all chess variants, a general modification of FEN is used for this (with a specific version of FEN for the base game of chess itself.) This version of FEN for the board state itself is the same as the typical chess FEN, with the following additions:

- The FEN variant can support more than 8 rows and 8 columns.
- The FEN variant can support custom pieces, by defining the custom piece symbol (eg. `A` for Archbishops) in the piece's `get_piece_symbol` method.
- Additional information can be specified for individual pieces.
    - If the `!` marker follows a piece (eg. `p!`), that piece has moved at least once already. This is a general way to handle things like first pawn moves and castling rights.
    - Like typical FEN, if a piece is uppercase it will be assumed as team zero (white), and if it's lowercase it will be assumed as team one (black.) However, you can specify an `{T}` marker after a given piece to specify its team.

For instance, `p!{3}` is a pawn that has moved once before on the fourth team. (We're using zero as the first index, much like arrays do in programming.)

The active team to move can be represented by either `w` (team 0) and `b` (team 1), or by the team number to move itself.

Then, the last UCI moves, specified by a `,` can be shown. For instance: `e2e4, e7e5` with moves earlier in the list having happened earlier. These would be the last two moves. How many moves are shown depends on the variant's rules, but this FEN representation should be compatible with any of them.

Finally, the halfmove clock and fullmove clocks are the same as traditional FEN notation.


## TODOs

### Indirection

As of right now, we store `Piece` as a `Box<dyn Piece>`, which introduces a layer of indirection. However, since we can guarantee the size of `Piece`, and we just want `Piece`'s functionality, we can optimize by writing our own vtable that removes the indirection.

_(Example provided by the very kind quicknir#3667 on the Rust Programming Language Community Server.)_
```rust
struct foo {
    first_method: fn(i32, i32) -> i32,
}

fn first_method_impl_one(x: i32, y: i32) -> i32 { x + y }

static impl_one: foo = foo{first_method: first_method_impl_one};

fn main() {
   let mut v = vec![];
   v.push(&impl_one);
}
```

## License

`monster-chess` available under the
[MIT license](https://opensource.org/licenses/MIT). See
[LICENSE](https://github.com/chesstastic-org/monster-chess/blob/main/LICENSE) for the full
license text.