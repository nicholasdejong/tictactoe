use colored::{control, ColoredString, Colorize};
// use core::{panic};
use std::io;

#[derive(PartialEq, Clone, Copy, Debug)]
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

#[derive(Debug)]
struct Root {
    //end of the branch
    score: isize,
    index: usize,
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
struct BoardState {
    row: [isize; 3],
    column: [isize; 3],
    diagonal: isize,  //     \
    cdiagonal: isize, //     /
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
// fn invert_turn(turn: &Turn) -> Turn {
//     match turn {
//         Turn::Computer => Turn::Player,
//         Turn::Player => Turn::Computer,
//     }
// }
fn coord_to_index(coord: String) -> usize {
    let letter = coord.chars().next().expect("Invalid coordinate provided");
    let ch_number: char = coord.chars().nth(1).unwrap();
    let number: usize = ch_number.to_digit(10).unwrap() as usize;
    // println!("{} {}", letter, number);
    let converted: usize = match letter {
        'a' => 0,
        'b' => 3,
        'c' => 6,
        _ => panic!("Invalid row-coordinate (abc)"),
    };
    converted + number - 1
}
fn index_to_coord(index: usize) -> String {
    let row = (index / 3) as usize;
    let col = index % 3;
    format!(
        "{}{}",
        match row {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            _ => panic!("invalid index"),
        },
        col + 1
    )
}
#[allow(dead_code)]
#[derive(Clone)]
struct GameState<'a> {
    moves: Vec<usize>,
    board: [Piece; 9],
    crosses: &'a Turn,
    naughts: &'a Turn, //dead btw
    state: BoardState,
    current_turn: Turn,
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
                diagonal: 0,
                cdiagonal: 0,
            },
            current_turn: *turn,
        };
    }
    fn get_piece(&self, turn: &Turn) -> Piece {
        if self.crosses == turn {
            Piece::Crosses
        } else {
            Piece::Naughts
        }
    }
    fn apply(&mut self, turn: &Turn, position: &usize) -> &Self {
        //X is the maximizing player and O is the minimizing player
        let piece = self.get_piece(turn);
        let row = (position / 3) as usize;
        let column = position % 3;
        let value = match self.get_piece(turn) {
            Piece::Crosses => 1,
            Piece::Naughts => -1,
            _ => 0, //impossible, must either be naughts or crosses
        };
        // println!("{:?}, {:?}", self.state.row, self.state.column);
        // println!("{:?}", piece);
        if self.board[*position] == Piece::Empty {
            self.board[*position] = piece;
            self.state.row[row] += value;
            self.state.column[column] += value;
            if position % 3 == (position / 3) as usize {
                self.state.diagonal += value;
            };
            if 2 - (position % 3) == (position / 3) as usize {
                self.state.cdiagonal += value;
            }
            //self.moves = self.moves.iter().filter(|m:&usize| m != &position).collect();
            self.moves.retain(|&m| &m != position);
        } else {
            panic!(
                "Invalid piece placement: {:?} (piece: {:?})",
                self.board, piece
            );
        }
        self
    }
    fn undo(&mut self, position: &usize) -> &Self {
        let piece = self.board[*position];
        let value: isize = match piece {
            Piece::Crosses => 1,
            Piece::Naughts => -1,
            Piece::Empty => {
                panic!("No move provided")
            }
        };
        self.board[*position] = Piece::Empty;
        self.moves.push(*position);
        let row = (position / 3) as usize;
        let column = position % 3;
        self.state.row[row] -= value;
        self.state.column[column] -= value;
        if position % 3 == (position / 3) as usize {
            self.state.diagonal -= value;
        };
        if 2 - (position % 3) == (position / 3) as usize {
            self.state.cdiagonal -= value;
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
        print!(
            "{}",
            format!(
                "
          1   2   3     
        +---+---+---+
      a | {} | {} | {} |    Tic-Tac-Toe
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
                format!("Red: Computer").bright_red()
            )
            .bright_cyan()
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
    fn winner(&self) -> Piece {
        for row in self.state.row {
            match row {
                3 => return Piece::Crosses,
                -3 => return Piece::Naughts,
                _ => continue,
            }
        }
        for col in self.state.column {
            match col {
                3 => return Piece::Crosses,
                -3 => return Piece::Naughts,
                _ => continue,
            }
        }
        match self.state.diagonal {
            3 => return Piece::Crosses,
            -3 => return Piece::Naughts,
            _ => {}
        }
        match self.state.cdiagonal {
            3 => return Piece::Crosses,
            -3 => return Piece::Naughts,
            _ => {}
        }
        return Piece::Empty;
    }
    fn full(&self) -> bool {
        self.moves.len() == 0
    }
    fn minimax(&self, is_max: bool) -> Root {
        // print!("Initial function call");
        // self.print();
        if self.full() || self.winner() != Piece::Empty {
            // println!("{:?}", self.winner());
            return Root {
                score: *&match self.winner() {
                    Piece::Crosses => 1,
                    Piece::Naughts => -1,
                    Piece::Empty => 0,
                },
                index: 9,
            };
        }
        if is_max {
            let mut best_move = 0usize;
            let mut best_score = -2isize;
            for current_move in &self.moves {
                let mut new_state = self.clone();
                new_state.apply(&new_state.crosses, &current_move);
                // self.print();
                // new_state.print();
                // panic!("e");
                let current_score = new_state.minimax(false).score;
                if current_score > best_score {
                    best_score = current_score;
                    best_move = *current_move;
                    // println!("New best: {}, {}", best_score, best_move);
                }
                new_state.undo(current_move);
                // new_state.print();
                // self.print();
                assert_eq!(&new_state.state, &self.state);
                // println!("{:?}", new_state.state)
            }
            // println!("res {}, {}", best_score, best_move);
            return Root {
                score: best_score,
                index: best_move,
            };
        } else {
            let mut best_move = 0usize;
            let mut best_score = 2isize;
            for current_move in &self.moves {
                let mut new_state = self.clone();
                new_state.apply(&new_state.naughts, &current_move);
                // self.print();
                // new_state.print();
                // panic!("e");
                let current_score = new_state.minimax(true).score;
                if current_score < best_score {
                    best_score = current_score;
                    best_move = *current_move;
                    // a = current_score;
                    // b = *current_move;
                    // println!("New best: {}, {}", best_score, best_move);
                }
                new_state.undo(current_move);
            }
            // println!("res {}, {}", best_score, best_move);
            return Root {
                score: best_score,
                index: best_move,
            };
        }
        // println!("{} {}", a, b);
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
    let computer_is_max = match state.crosses {
        &Turn::Player => false,
        &Turn::Computer => true,
    };
    for _ in 0..9 {
        if state.winner() != Piece::Empty {
            match state.winner() {
                Piece::Crosses => {
                    println!("X wins!")
                }
                Piece::Naughts => {
                    println!("O wins!")
                }
                _ => {}
            }
            break;
        }
        if &state.current_turn == &Turn::Computer {
            let best = state.minimax(computer_is_max);
            state.apply(&Turn::Computer, &best.index);
            state.current_turn = Turn::Player;
            println!("Score: {}", best.score);
        } else {
            state.print();
            let moves: Vec<String> = state
                .moves
                .iter()
                .map(|m| index_to_coord(*m) as String)
                .collect();
            println!("Choose a move! \n(available: {})", moves.join(", "));
            ln = String::new();
            io::stdin().read_line(&mut ln).expect("Failed to read line");
            let position = coord_to_index(ln);
            state.apply(&Turn::Player, &position);
            state.current_turn = Turn::Computer;
        }
    }
    // state.apply(&turn, &0);
    // state.apply(&invert_turn(&turn), &0);
    // state.apply(&turn, &1);
    // state.apply(&invert_turn(&turn), &1);
    // state.apply(&turn, &2);
    // state.apply(&invert_turn(&turn), &2);
    // state.apply(&turn, &3);
    // state.apply(&invert_turn(&turn), &3);
    // state.apply(&turn, &4);
    // state.apply(&invert_turn(&turn), &4);
    // state.apply(&turn, &5);
    // state.apply(&invert_turn(&turn), &5);
    // state.apply(&turn, &6);
    // state.apply(&invert_turn(&turn), &6);
    // state.apply(&turn, &7);
    // state.apply(&invert_turn(&turn), &7);
    // state.apply(&turn, &8);
    // state.apply(&invert_turn(&turn), &8);
    // state.print();
    // println!("{:?}", state.minimax(false));
    // ln = String::new();
    // io::stdin().read_line(&mut ln).expect("Failed to read line");
    // println!(":{}", ln);
    // println!("{}", coord_to_index(ln));
    // println!("{:?}, {:?}", state.moves, state.board);
}
