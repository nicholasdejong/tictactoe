use colored::{ColoredString, Colorize, control};
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
fn coord_to_index(coord: String) -> usize {
    let letter = coord.chars().next().expect("Invalid coordinate provided");
    let ch_number:char = coord.chars().nth(1).unwrap();
    let number:usize = ch_number.to_digit(10).unwrap() as usize;
    // println!("{} {}", letter, number);
    let converted: usize = match letter {
        'a' => 0,
        'b' => 3,
        'c' => 6,
        _ => panic!("Invalid row-coordinate (abc)"),
    };
    converted + number - 1
}
#[allow(dead_code)]
struct GameState<'a> {
    moves: Vec<usize>,
    board: [Piece; 9],
    crosses: &'a Turn,
    naughts: &'a Turn, //dead btw
    state: BoardState,
}

impl GameState<'_> {
    fn new(turn: &Turn) -> GameState {
        return GameState {
            moves: vec![0, 1, 2, 3, 4, 5, 6, 7, 8],
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
            //self.moves = self.moves.iter().filter(|m:&usize| m != &position).collect();
            self.moves.retain(|&m| m != position);
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
            Turn::Player => format!("X").bright_green(),
            Turn::Computer => format!("X").bright_red(),
        };
        let o = match self.naughts {
            Turn::Player => format!("O").bright_green(),
            Turn::Computer => format!("O").bright_red(),
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
        print!("{}",
            format!("
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
            format!("Green: You").bright_green(),
            colored[6],
            colored[7],
            colored[8],
            format!("Red: Computer").bright_red()).bright_cyan()
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
    fn done(&self) -> bool {
        self.moves.len() == 0
    }
}

fn main() {
    let _enabled = control::set_virtual_terminal(true);
    let mut ln = String::new();
    // println!("{}", format!("Test").green().on_blue());
    println!("Who should go first? Player or Computer? (p/c)");
    io::stdin().read_line(&mut ln).expect("Failed to read line");

    let turn: Turn = match ln.to_lowercase().chars().next().unwrap() {
        'p' => Turn::Player,
        'c' => Turn::Computer,
        _ => panic!("Please provide either 'player' or 'computer'!"),
    };

    let mut state: GameState = init(&turn);
    state.apply(&turn, 0);
    // state.apply(&invert_turn(&turn), 1);
    state.apply(&turn, 2);
    // state.apply(&invert_turn(&turn), 3);
    state.apply(&turn, 4);
    state.apply(&invert_turn(&turn), 5);
    // state.apply(&turn, 6);
    state.apply(&invert_turn(&turn), 7);
    state.apply(&turn, 8);
    state.print();
    println!("{}", state.done());
    ln = String::new();
    io::stdin().read_line(&mut ln).expect("Failed to read line");
    // println!(":{}", ln);
    println!("{}", coord_to_index(ln));
    // println!("{:?}, {:?}", state.moves, state.board);
}
