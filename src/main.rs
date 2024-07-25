use crate::cell::{Cell, CellState};

mod cell;
mod universe;
mod test_cell;
mod cell_rc;

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
