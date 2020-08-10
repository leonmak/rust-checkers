use super::board::{Coordinate, GamePiece, Move, PieceColor};
use super::board::PieceColor::{Black, White};

pub struct GameEngine {
    board: [[Option<GamePiece>; 8]; 8],
    current_turn: PieceColor,
//    move_count: u32,
}

pub struct MoveResult {
    pub mv: Move,
    pub crowned: bool, 
}

impl GameEngine {

    pub fn new() -> GameEngine {
        let mut engine = GameEngine {
            board: [[None; 8]; 8],
            current_turn: PieceColor::Black,
            // move_count: 0,
        };
        engine.initialize_pieces();
        engine
    }

    pub fn initialize_pieces(&mut self) {
        [1, 3, 5, 7, 0, 2, 4, 6, 1, 3, 5, 7].iter()
        .zip([0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2].iter())
        .map(|(a, b)| (*a as usize, *b as usize))
        .for_each(|(x, y)| {
            self.board[x][y] = Some(GamePiece::new(PieceColor::White));
        });

        [0, 2, 4, 6, 1, 3, 5, 7, 0, 2, 4, 6].iter()
        .zip([5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 7].iter())
        .map(|(a, b)| (*a as usize, *b as usize))
        .for_each(|(x, y)| {
            self.board[x][y] = Some(GamePiece::new(PieceColor::Black));
        });
    }

    pub fn get_piece(&self, coord: Coordinate) -> Result<Option<GamePiece>, ()> {
        let Coordinate(x, y) = coord;
        Ok(self.board[x][y])
    }

    pub fn current_turn(&self) -> PieceColor {
        return self.current_turn;
    }

    fn midpiece_coordinate(&self, fx: usize, fy: usize, tx: usize, ty: usize)-> Option<Coordinate> {
        let dx = tx as i8 - fx as i8;
        let dy = ty as i8 - fy as i8;
        Some(Coordinate(fx + (dx/2) as usize, fy + (dy/2) as usize))
    }

    fn should_crown(&self, piece: GamePiece, coord: Coordinate) -> bool {
        match piece.color {
            Black => coord.0 == 0,
            White => coord.0 == 7,
        }
    }

    fn crown_piece(&mut self, coord: Coordinate) {
        let Coordinate(x, y) = coord;
        if let Some(piece) = self.board[x][y] {
            self.board[x][y] = Some(GamePiece::crowned(piece));
        }
    }

    fn advance_turn(&mut self) {
        self.current_turn = if self.current_turn == White { Black } else { White };
    }

    pub fn move_piece(&mut self, mv: &Move) -> Result<MoveResult, ()> {
        let legal_moves = self.legal_moves();
        if !legal_moves.contains(mv) {
            return Err(());
        }
        let Coordinate(fx, fy) = mv.from;
        let Coordinate(tx, ty) = mv.to;
        let piece = self.board[fx][fy].unwrap(); // unwrap to get value in Option
        let midpiece_coordinate = self.midpiece_coordinate(fx, fy, tx, ty);
        if let Some(Coordinate(x, y)) = midpiece_coordinate {
            self.board[x][y] = None; // remove the jumped piece
        }
        // Move piece from source to dest
        self.board[tx][ty] = Some(piece);
        self.board[fx][fy] = None;
        let crowned = if self.should_crown(piece, mv.to) {
            self.crown_piece(mv.to);
            true
        } else {
            false
        };
        self.advance_turn();
        Ok(MoveResult {
            mv: mv.clone(),
            crowned: crowned,
        })
    }

    fn legal_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        for col in 0..8 {
            for row in 0..8 {
                // if-let instead of match optional { Some(i) => { print(i) } }
                if let Some(piece) = self.board[col][row] {
                    if piece.color == self.current_turn {
                        let loc = Coordinate(col, row);
                        let mut vmoves = self.valid_moves_from(loc);
                        moves.append(&mut vmoves);
                    }
                }
            }
        }
        moves
    }

    fn valid_jump(&self, p: &GamePiece, loc: &Coordinate, to: &Coordinate) -> bool {
        let Coordinate(fx, fy) = loc;
        let Coordinate(tx, ty) = to;
        let dx = if tx > fx { tx - fx } else { fx - tx };
        let up = ty > fy;
        let dy = if up { ty - fy } else { fy - ty };
        if !p.crowned {
            if !up && p.color == White {
                return false;
            }
            if up && p.color == Black {
                return false;
            }
        }
        dx <= 2 && dy <= 2 && to.on_board()
    }

    fn valid_move(&self, p: &GamePiece, loc: &Coordinate, to: &Coordinate) -> bool {
        let Coordinate(x, y) = *to;
        self.valid_jump(p, loc, to)
            && self.current_turn == p.color
            && self.board[x][y].is_none()
    }

    fn valid_moves_from(&self, loc: Coordinate) -> Vec<Move> {
        let Coordinate(x, y) = loc;
        if let Some(p) = self.board[x][y] {
            let mut jumps = loc
                .jump_targets_from()
                .filter(|t| self.valid_jump(&p, &loc, &t))
                .map(|ref t| Move {
                    from: loc.clone(),
                    to: t.clone(),
                })
                .collect::<Vec<Move>>();
            let mut moves = loc
                .move_targets_from()
                .filter(|t| self.valid_move(&p, &loc, &t))
                .map(|ref t| Move {
                    from: loc.clone(),
                    to: t.clone(),
                })
                .collect::<Vec<Move>>();
            jumps.append(&mut moves);
            jumps
        } else {
            Vec::new()
        }
    }

}

