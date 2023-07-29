use rusty_engine::prelude::*; // Prelude is one of the three usages of import * in Rust

fn main() {
    let mut game = Game::new();

    game.run(());
}
