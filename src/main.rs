mod lib;

use lib::Game;

fn main() {
    let mut game: Game = Game::new();
    game = game.try_move(0, 3);
    game = game.try_move(0, 3);

    println!("Current game board: \n{}", game);
}
