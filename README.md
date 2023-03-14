<div align = "center">
<h1>monster-chess</h1>
</div>

## Overview

`monster-chess` is a fairy chess move generation library written in Rust. It was created as part of the Chesstastic project (which aims to allow for users to easily create chess variants, play against others in those chess variants and analyze games of said variants), and is meant to handle move generation and move validation logic that happens in those variants. Currently, the library isn't functional, but hopefully it will be in the near future.

### Compatibility

When we say that we're compatible with a given game, that doesn't mean we'll necessarily be providing out-of-the-box support for it, but that the game will be implementable given the framework that `monster-chess` provides.

Types of games that we're aiming to be compatible with:
- [Chess](https://en.wikipedia.org/wiki/Chess)
- [Fischer Random Chess](https://en.wikipedia.org/wiki/Fischer_random_chess)
- [Shogi](https://en.wikipedia.org/wiki/Shogi)
- [Maharajah and the Sepoys](https://en.wikipedia.org/wiki/Maharajah_and_the_Sepoys)
- [Ataxx](https://en.wikipedia.org/wiki/Ataxx)

Types of games we *may* aim to be compatible with:
- [Go](https://en.wikipedia.org/wiki/Go_(game))

Types of games we're not aiming to be compatible with:
- [5D Chess with Multiversal Time Travel](https://store.steampowered.com/app/1349230/5D_Chess_With_Multiverse_Time_Travel/)
- [Minecraft](https://en.wikipedia.org/wiki/Minecraft)

However, the following games _will_ be supported out of the box:
- [Chess](https://en.wikipedia.org/wiki/Chess)
- [Fischer Random Chess](https://en.wikipedia.org/wiki/Fischer_random_chess)
- [Ataxx](https://en.wikipedia.org/wiki/Ataxx)

If you're wondering if a given game or chess variant is compatible with chess, imagine starting with the base game of chess, and see if you can do any of the following to get to your variant.

- Can I increase/decrease the board size?
- Can I remove existing pieces, or add new pieces with new types of movement?
- Can I change the capture style of pieces (eg. swapping positions with the piece instead of capturing it.)
- Can I change the movement restrictions? (eg. being allowed to move into check)
- Can I change the win conditions? (eg. being able to win by capturing all pieces.)

If so, `monster-chess` most likely will be able to be compatible with said variant, but you may have to add it.

## Out of the Box Support

### Chess

Chess is a two-player game where both players start with a color and the same set of pieces, and must engage in a fruitful battle until one side is victorious. For the sake of brevity, we'll assume you're well-acquainted with the rules of chess (if not, feel free to [read about them here](https://www.chess.com/terms/chess)), and simply focus on the parts relevant to `monster-chess`.

![Ladder Checkmate](https://i.imgur.com/xGu1ODZ.jpg)

In chess, the end-goal of the game is **checkmate**, where the opponent's king is in check (in the line of sight of another piece), and has no moves that can escape said check (moving out of the way, blocking with another piece, etc.) The image above demonstrates this, with two rooks blocking all of the king's movement squares, and the opponent's king is simply unable to move out of the line of sight of the rooks. If the opponent is in check but can escape it, they must, and you cannot put yourself in check.

The way `monster-chess` handles this is using **pseudolegal** move generation. First, we generate all of the moves any piece would be able to make (aka. pseudolegal moves), and then we check to make sure your king isn't in check after you perform the move. To do this, we have to generate _every possible move_ that the opponent can make after your move, and see if one of them allows for the king to be captured. This is expensive, but we have two ways to make this faster.
- For pieces that move and capture differently, we only focus on the squares they capture.
- For pieces that move in one direction until they hit another piece or the edge of the board (bishops, rooks, and queens), we check if the king is even in the line of sight of the piece before seeing if it's truly in the line of sight.

Despite all of that, this legality check is rather expensive. Fortunately, it won't matter much if you're building a [chess engine](https://www.chessprogramming.org/Engines), as in any given position, you'll only be searching a fraction of the moves. This means, instead of performing the legality check for every move in the position, if you search 3 moves and find an instant hit, you won't need to check the rest.

You can initialize the chess board as follows:

```rust
    // We have to store `chess` in a variable and borrow it because `monster-chess` separates the game instance from the actual board-state itself to avoid an extra runtime cost in many cases.
    let chess = Chess::create();
    let mut board = chess.from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
```

Then, we can generate the moves as follows:

```rust
// We have to specify the mode because `ATTACKS_MODE` is used to optimizing checks, and we don't know what mode to generate for until you specify it.
let legal_moves = board.generate_legal_moves(NORMAL_MODE);
let pseudolegal_moves = board.generate_moves(NORMAL_MODE);
```

For testing and benchmarking purposes, `monster-chess` provides a method named `perft`, which will count the number of all possible moves possible that are `depth` half-moves ahead from the position (with a half-move being a move from one of the two players for reference.)

```rust
    let perft = board.perft(5, true);
    let perft_pseudolegal = board.perft(5, false);
```

From the benchmarks I've done, `monster-chess` can reach about 20,000,000 pseudo-legal moves per second, and 5,000,000 legal moves per second. This isn't ideal and if you're only interested in performance, I recommend using the [cozy-chess](https://github.com/analog-hors/cozy-chess/) crate which is at least 25x faster then the implementation of chess in `monster-chess`. However, `monster-chess` is a sound option for chess given you also want the ability to support chess variants or even other games.

It may be noted that `monster-chess` also aims to support [Fischer Random Chess](https://www.chess.com/terms/chess960). As of now, Fischer Random Chess is theoretically supported in the implementation of `monster-chess`'s Chess implementation, but as of now, it isn't tested, and FENs for the variant aren't supported. It would be trivial to add it in the framework of `monster-chess` as an extension of the existing Chess implementation, though.

### Ataxx

This game will be supported in the very-near future.

## Implementation

### Bitboards

`monster-chess` uses a general implementation of [Bitboards](https://www.chessprogramming.org/Bitboards) to extend to larger board sizes, using a custom made `BitBoard` data type.

```rust
pub struct BitBoard<const T : usize> {
    pub data: [ u128; T ]
}
```

BitBoards are designed in such a way that if the BitBoard only has to store one `u128`, it would (ideally) be optimized to be as fast as it would natively. BitBoards support all **bitwise** operators, alongside addition, subtraction, and two custom methods for [bitscans](https://www.chessprogramming.org/BitScan).

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
    fn get_moves(&self, board: &Board, from: BitBoard, team: u32) -> BitBoard {
        let lookup = self.get_attack_lookup(board);
        match lookup {
            Some(lookup) => lookup[from.bitscan_reverse() as usize][0],
            None => self.generate_moves(board, from)[0]
        }
    }
}
```

### FEN Representation

#### Board State

Because `monster-chess` is aiming to support all chess variants, a general modification of FEN is used for this (with a specific version of FEN for the base game of chess itself.) This version of FEN for the board state itself is the same as the typical chess FEN, with the following additions:

- The FEN variant can support more or less than 8 rows and 8 columns.
- The FEN variant can support custom pieces, by defining the custom piece symbol (eg. `A` for Archbishops) in the piece's `get_piece_symbol` method.
- Additional information can be specified for individual pieces.
    - There are two ways to specify the piece type and the team, depending on the individual piece.
        - `PieceSymbol::Char` defines the piece as a single char (eg. `p`.) If there are two teams, `P` will represent the first team (team `0`), and `p` will represent the second team (team `1`.) If there are more than two teams, the teams will be represented with braces after the piece. (eg. `p{2}` for team three.)
        - `PieceSymbol::Teams` changes what char is used for the piece depending on the team. (eg. `x` for player one, `o` for player two.)
    - If a game supports first move notation, then if the `!` marker follows a piece (eg. `p!`), that piece has moved at least once already. This is a general way to handle things like first pawn moves and castling rights.

For instance, `p!{3}` is a pawn that has moved once before on the fourth team. (We're using zero as the first index, much like arrays do in programming.)

There's only one option for FEN States, and that's `first_move`. In most games, first moves don't have any impact on the game state, or the FEN representation has other, more concise ways to represent first moves (eg. in chess, pawn first moves are detected based on if pawns are on the 2nd or 6th ranks.)

#### Fen Arguments

FEN Notation for games like chess also have additional information provided that isn't in the board state representation itself. For instance, the en passant square, or castling rights, or the side to move. `monster-chess` does not support these natively as part of the `Board` implementation. Instead, individual games have to manage the additional arguments for their respective FEN notations themselves, by implementing the `FenArgument` trait.

`FenArgument` provides two main methods, `encode`, and `decode`.

```rust
pub trait FenArgument {
    /// `encode` takes in a board, and outputs what this FEN argument's encoded result would be (eg. for a team argument, it could be `"b"`)
    fn encode(&self, board: &Board) -> String;

    /// `decode` takes in a board and an existing argument, and will modify the board to meet the argument (eg. changing the team to reflect the given arg team of `w`)
    fn decode(&self, board: &mut Board, arg: &str) -> Result<(), FenDecodeError>;
}
```

`monster-chess` provides an implementation of one argument for you, which is `FenTeamArgument`, defined like this:

```rust
pub enum FenTeamArgument {
    Number,
    Teams(Vec<char>),
}
```

If your game needs an argument to represent which side has to move (which it almost certainly does), using `FenTeamArgument` is necessary, unless you decide to define your own argument representing which side has to move.

### Games

`monster-chess` finally provides a struct called `Game`, which is used to describe the rules of your chess-adjacent game. It would be declared as follows:

```rust
pub struct Game {
    pub pieces: Vec<&'static dyn Piece>,
    pub move_restrictions: Box<dyn MoveRestrictions>, // Pseudo-legal Move Generation
}
```

## License

`monster-chess` available under the
[MIT license](https://opensource.org/licenses/MIT). See
[LICENSE](https://github.com/chesstastic-org/monster-chess/blob/main/LICENSE) for the full
license text.
