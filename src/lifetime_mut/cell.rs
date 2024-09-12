use rand::Rng;
use crate::common::cell_state::CellState;
use crate::common::relative_position::RelativePosition;

#[derive(Debug)]
pub struct Cell<'a> {
    state: CellState
}

impl<'a> Cell<'a> {
    pub fn is_alive(&self) -> bool {
        return match self.state {
            CellState::ALIVE => { true }
            CellState::DEAD => { false }
        };
    }

    pub fn print(&self) -> String {
        let print = match self.is_alive() {
            true => { "x".to_string() }
            false => { "o".to_string() }
        };
        format!("{}", print)
    }

    pub fn new(state: &CellState) -> Cell<'a> {
        Cell {
            state: match state {
                CellState::ALIVE => CellState::ALIVE,
                CellState::DEAD => CellState::DEAD
            }
        }
    }

    pub fn new_random_state() -> Cell<'a> {
        let state = rand::thread_rng().gen_range(0..2);
        match state {
            0 => Cell::new_dead(),
            _ => Cell::new_alive()
        }
    }

    pub fn new_alive() -> Cell<'a> {
        Cell {
            state: CellState::ALIVE,
        }
    }

    pub fn new_dead() -> Cell<'a> {
        Cell {
            state: CellState::DEAD,
        }
    }
}

#[cfg(test)]
mod cell_tests {
    use super::*;

    #[test]
    fn should_be_alive_at_next_tick_when_alive() {
        let cell = Cell::new_alive();

        cell.tick();

        assert_eq!(cell.is_alive(), true);
    }
}