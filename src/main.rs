use colored::{ColoredString, Colorize};
use core::panic;
use std::io;

#[derive(PartialEq)]
enum Turn {
    Player,
    Computer,
}

#[derive(Clone, Debug, PartialEq, Copy)]
enum Piece {
    Naughts,
    Crosses,
    Empty,
}

#[allow(dead_code)]
struct BoardState {
    row: [isize; 3],
    column: [isize; 3],
    diagonal: [isize; 3],
    cdiagonal: [isize; 3], //counter diagonal (opposite)
}

fn init(turn: &Turn) -> GameState {
    GameState::new(turn)
}
// fn piece_char(piece: &Piece) -> char {
//     match piece {
//         Piece::Crosses => 'X',
//         Piece::Naughts => 'O',
//         Piece::Empty => '#',
//     }
// }
fn invert_turn(turn: &Turn) -> Turn {
    match turn {
        Turn::Computer => Turn::Player,
        Turn::Player => Turn::Computer,
    }
}
#[allow(dead_code)]
struct GameState<'a> {
    moves: [usize; 9],
    board: [Piece; 9],
    crosses: &'a Turn,
    naughts: &'a Turn, //dead btw
    state: BoardState,
}

impl GameState<'_> {
    fn new(turn: &Turn) -> GameState {
        return GameState {
            moves: [0, 1, 2, 3, 4, 5, 6, 7, 8],
            board: [Piece::Empty; 9],
            crosses: turn,
            naughts: match &turn {
                Turn::Player => &Turn::Computer,
                Turn::Computer => &Turn::Player,
            },
            state: BoardState {
                row: [0; 3],
                column: [0; 3],
                diagonal: [0; 3],
                cdiagonal: [0; 3],
            },
        };
    }
    fn get_piece(&self, turn: &Turn) -> Piece {
        if self.crosses == turn {
            Piece::Crosses
        } else {
            Piece::Naughts
        }
    }
    fn apply(&mut self, turn: &Turn, position: usize) -> &Self {
        //X is the maximizing player and O is the minimizing player
        let piece = self.get_piece(turn);
        let row = (position / 3) as usize;
        let column = position % 3;
        let value = match self.get_piece(turn) {
            Piece::Crosses => 1,
            Piece::Naughts => -1,
            _ => 0, //impossible, must either be naughts or crosses
        };
        println!("{:?}, {:?}", self.state.row, self.state.column);
        println!("{:?}", piece);
        if self.board[position] == Piece::Empty {
            self.board[position] = piece;
            self.state.row[row] += value;
            self.state.column[column] += value;
        } else {
            panic!(
                "Invalid piece placement: {:?} (piece: {:?})",
                self.board, piece
            );
        }
        self
    }

    fn print(&self) {
        let x = match self.crosses {
            Turn::Player => format!("X").green(),
            Turn::Computer => format!("X").red(),
        };
        let o = match self.naughts {
            Turn::Player => format!("O").green(),
            Turn::Computer => format!("O").red(),
        };
        let space = format!(" ").black();
        let colored: Vec<&ColoredString> = self
            .board
            .iter()
            .map(|p: &Piece| match p {
                Piece::Crosses => &x,
                Piece::Naughts => &o,
                Piece::Empty => &space,
            })
            .collect();
        print!(
            "
          1   2   3     
        +---+---+---+
      a | {} | {} | {} |    Tic-Tac-Toe by Nicholas
        +---+---+---+
      b | {} | {} | {} |    {}
        +---+---+---+
      c | {} | {} | {} |    {}
        +---+---+---+
        ",
            colored[0],
            colored[1],
            colored[2],
            colored[3],
            colored[4],
            colored[5],
            format!("Green: You").green(),
            colored[6],
            colored[7],
            colored[8],
            format!("Red: Computer").red()
        )
        // let mut cln = String::new();
        // let mut char_list: Vec<char> = vec![];
        // for p in self.board {
        //     char_list.push(piece_char(&p));
        // }
        // /*
        // for ch in char_list {
        //     println!("{}", ch);
        // }
        // */
        // for cnt in 0..char_list.len() {
        //     if cnt % 3 == 0 {
        //         println!("{}", cln);
        //         cln = String::new();
        //     };
        //     cln.push(char_list[cnt]);
        // }
        // println!("{}", cln);
    }
    // fn determine_winner(&self) -> Side {
    //     if self.board

    // }
}

fn main() {
    let mut ln = String::new();
    println!("{}", format!("Test").green());
    println!("Who should go first? Player or Computer? (p/c)");
    io::stdin().read_line(&mut ln).expect("Failed to read line");

    let turn: Turn = match ln.to_lowercase().chars().next().unwrap() {
        'p' => Turn::Player,
        'c' => Turn::Computer,
        _ => panic!("Please provide either 'player' or 'computer'!"),
    };

    let mut state: GameState = init(&turn);
    state.apply(&turn, 0);
    state.apply(&invert_turn(&turn), 1);
    state.apply(&turn, 2);
    state.apply(&invert_turn(&turn), 3);
    state.apply(&turn, 4);
    state.apply(&invert_turn(&turn), 5);
    state.apply(&turn, 6);
    state.apply(&invert_turn(&turn), 7);
    state.apply(&turn, 8);
    state.print();
    // println!("{:?}, {:?}", state.moves, state.board);
}
