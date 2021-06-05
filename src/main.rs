use core::fmt::{self, Display};
use std::io;

const BOARD_WIDTH: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Piece {
    Naught,
    Cross,
}

impl Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &Piece::Cross => write!(f, "X"),
            &Piece::Naught => write!(f, "O"),
        }
    }
}

type BoardPosition = Option<Piece>;

#[derive(Debug, Clone)]
struct Board {
    positions: Vec<BoardPosition>,
}

impl Board {
    fn new() -> Self {
        Board {
            positions: (0..BOARD_WIDTH * BOARD_WIDTH)
                .into_iter()
                .map(|_i| None)
                .collect(),
        }
    }

    fn get_position(&self, x: usize, y: usize) -> Result<BoardPosition, TicTacToeError> {
        if x >= BOARD_WIDTH || y >= BOARD_WIDTH {
            return Err(TicTacToeError::OutOfBounds);
        }
        Ok(self.positions[y * BOARD_WIDTH + x])
    }

    fn set_piece(&self, x: usize, y: usize, piece: Piece) -> Result<Self, TicTacToeError> {
        if let Some(_piece) = self.get_position(x, y)? {
            return Err(TicTacToeError::PieceInPosition(x, y));
        }
        Ok(Board {
            positions: self
                .clone()
                .positions
                .into_iter()
                .enumerate()
                .map(|(i, position)| {
                    if i == y * BOARD_WIDTH + x {
                        Some(piece)
                    } else {
                        position
                    }
                })
                .collect(),
            ..*self
        })
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..BOARD_WIDTH {
            for j in 0..BOARD_WIDTH {
                match self.positions[i * BOARD_WIDTH + j] {
                    None => write!(f, ".")?,
                    Some(piece) => write!(f, "{}", piece)?,
                }
                write!(f, " ")?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[derive(Debug)]
enum TicTacToeError {
    OutOfBounds,
    PieceInPosition(usize, usize),
    InvalidMoveInput,
}

#[derive(Debug)]
struct GameState {
    board: Board,
    turn: Piece,
}

#[derive(Debug)]
struct Game {
    state: GameState,
    winning_positions: Vec<Vec<(usize, usize)>>,
}

impl Game {
    fn new() -> Self {
        Game {
            state: GameState {
                board: Board::new(),
                turn: Piece::Naught,
            },
            winning_positions: (0..BOARD_WIDTH)
                .into_iter()
                .flat_map(|i| {
                    {
                        vec![
                            (0..BOARD_WIDTH).into_iter().map(move |j| (j, i)).collect(),
                            (0..BOARD_WIDTH).into_iter().map(move |j| (i, j)).collect(),
                        ]
                    }
                })
                .chain(vec![
                    (0..BOARD_WIDTH).into_iter().map(|i| (i, i)).collect(),
                    (0..BOARD_WIDTH)
                        .into_iter()
                        .map(|i| (BOARD_WIDTH - 1 - i, i))
                        .collect(),
                ])
                .collect(),
        }
    }

    fn reset_state(&self) -> GameState {
        GameState {
            board: Board::new(),
            turn: self.change_turn(),
        }
    }

    fn display_board(&self) {
        println!("{}", self.state.board);
    }

    fn change_turn(&self) -> Piece {
        match self.state.turn {
            Piece::Naught => Piece::Cross,
            Piece::Cross => Piece::Naught,
        }
    }

    fn get_move(&self) -> Result<(usize, usize), TicTacToeError> {
        let mut position_str_raw = String::new();
        io::stdin()
            .read_line(&mut position_str_raw)
            .map_err(|_| TicTacToeError::InvalidMoveInput)?;
        let coords = position_str_raw.trim().split(" ").collect::<Vec<_>>();
        Ok((
            match coords[0].parse::<usize>() {
                Ok(i) => i,
                Err(..) => return Err(TicTacToeError::InvalidMoveInput),
            },
            match coords[1].parse::<usize>() {
                Ok(i) => i,
                Err(..) => return Err(TicTacToeError::InvalidMoveInput),
            },
        ))
    }

    fn check_slice_for_winner(&self, slice: &Vec<(usize, usize)>) -> Option<Piece> {
        let positions: Vec<BoardPosition> = slice
            .iter()
            .map(|(x, y)| {
                self.state.board.get_position(*x, *y).expect(
                    format!(
                        "winning positions have invalid coords in them: {}, {}",
                        x, y
                    )
                    .as_str(),
                )
            })
            .collect();
        let position = positions
            .get(0)
            .expect("winning positions have slices of length 0");
        if positions.iter().all(|p| p.is_some() && p == position) {
            return *position;
        }
        None
    }

    fn check_winner(&self) -> Option<Piece> {
        for slice in self.winning_positions.iter() {
            if let Some(winner) = self.check_slice_for_winner(slice) {
                return Some(winner);
            }
        }
        return None;
    }

    fn run(mut self) {
        println!("Starting the game!");
        loop {
            loop {
                self.display_board();
                println!("Pick a position:");
                loop {
                    match {
                        self.get_move()
                            .and_then(|(x, y)| self.state.board.set_piece(x, y, self.state.turn))
                    } {
                        Ok(board) => {
                            self.state.board = board;
                            break;
                        }
                        Err(_) => println!("Invalid move. Try again:"),
                    };
                }
                if let Some(winner) = self.check_winner() {
                    println!("{} won!", winner);
                    break;
                }
                self.state.turn = self.change_turn();
            }
            println!("Starting a new game!");
            self.state = self.reset_state();
        }
    }
}

fn main() {
    Game::new().run();
}
