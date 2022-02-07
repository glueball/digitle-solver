extern crate core;

use crate::solve::{solve, Game};

mod candidate;
mod number_list;
mod operation;
mod solve;

fn main() {
    // // Unsolvable!
    // let game = Game::new(482, vec![50, 25, 6, 5, 10]);
    // solve(game);

    let game = Game::new(508, vec![6,1,7,8,9]);
    solve(game);
}
