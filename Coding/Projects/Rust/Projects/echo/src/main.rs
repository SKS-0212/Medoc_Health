mod feature;

use crate::feature::game::game_loop::{initialize, run};

fn main() {
    println!("ECHO - The Last Signal");
    initialize();
    run();
}
