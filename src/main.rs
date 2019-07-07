use std::fmt;

#[derive(Copy, Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
enum Color {
    Black,
    White,
}

type Board = [[Option<Color>; 9]; 9];

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str: String = String::new();

        for row in self.board.iter() {
            for cell in row.iter() {
                match cell {
                    None => str.push_str("empty, "),
                    Some(color) => {
                        let color_str = match color {
                            Color::Black => "black, ",
                            Color::White => "white, ",
                        };
                        str.push_str(color_str)
                    }
                }
            }
            str.push_str("\n");
        }

        write!(f, "{}", str)
    } 
}

struct Game {
    board: Board,
    current_turn: Color,
}

impl Game {
    fn new() -> Game {
        Game {
            board: [
                [None, None, None, None, None, None, None, None, None,],
                [None, None, None, None, None, None, None, None, None,],
                [None, None, None, None, None, None, None, None, None,],
                [None, None, None, None, None, None, None, None, None,],
                [None, None, None, None, None, None, None, None, None,],
                [None, None, None, None, None, None, None, None, None,],
                [None, None, None, None, None, None, None, None, None,],
                [None, None, None, None, None, None, None, None, None,],
                [None, None, None, None, None, None, None, None, None,],
            ],
            current_turn: Color::Black,
        }
    }

    fn try_move(mut self, x: usize, y: usize) -> Game {
        let next_turn = match self.current_turn {
            Color::Black => Color::White,
            Color::White => Color::Black,
        };

        match self.board[x][y] {
            Some(_) => self,
            None => {
                self.board[x][y] = Some(self.current_turn);
                self.current_turn = next_turn; 
                self
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_move() {
        // black is the first to go:
        let g = Game::new();
        assert_eq!(g.current_turn, Color::Black);
        // try_move prevents overriding existing piece
        let g = Game::new();
        let g = g.try_move(0, 1);
        let g = g.try_move(0, 1);
        // still white's turn after failed move:
        assert_eq!(g.current_turn, Color::White);
        assert_eq!(g.board[0][1], Some(Color::Black));
    }
}

fn main() {
    let mut game: Game = Game::new();
    game = game.try_move(0, 3);
    game = game.try_move(0, 3);

    println!("Current game board: \n{}", game);
}
