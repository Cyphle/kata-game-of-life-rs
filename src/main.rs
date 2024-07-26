use crate::cell::{Cell, CellState};

mod universe;
mod cell;

fn main() {
    // TODO
    /*
    Faisons le en objet pour apprendre l'objet avec Rust
    - Cell with state (alive/dead)
    - Universe
     */

    let x = Cell::new(CellState::ALIVE);

    println!("Hello, world!");
}
