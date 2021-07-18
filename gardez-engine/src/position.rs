use crate::bitboard::Bitboard;
use crate::core::Color;
use crate::core::Piece;
use crate::core::Square;
use crate::core::N_COLORS;
use crate::core::N_ROLES;
use crate::core::N_SQUARES;

pub struct Position {
    state: State,
    board: Board,
}

struct State {
    color_to_move: Color,
    white_ooo_castle: bool,
    white_oo_castle: bool,
    black_ooo_castle: bool,
    black_oo_castle: bool,
    half_move_clock: u8,
    full_move_count: u32,
    en_passant_target: Option<Square>,
}

struct Board {
    pub squares: [Option<Piece>; N_SQUARES as usize],
    pub color_bitboards: [Bitboard; N_COLORS as usize],
    pub piece_bitboards: [Bitboard; N_ROLES as usize],
}

/*impl FromStr for Position {
type Err = &str::ParseError;

fn from_str(fen: &str) -> Result<Self, Self::Err> {
    let mut state = State::new();
    let mut board = Board::new();

    let tokens = fen.split(' ').collect();

    // 1. Place pieces on the board.
    let ranks = tokens.next().split('/');
    if ranks.len() != 8 {
        return Err("Invalid FEN string");
    }


    // 2. Set the color of the next player to move.

    // 3. Set castling rights.

    // 4. Set the en passant square.

    // 5. Set the number of half-moves since the last pawn advance.

    // 6. Set the full move number.


    Ok(Position{state, board})
}*/
