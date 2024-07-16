pub enum CellState {
    ALIVE,
    DEAD
}

pub enum RelativePosition {
    NORTH,
    EAST,
    SOUTH,
    WEST
}

pub struct Cell {
    state: CellState,
    neighbours: Vec<(Cell, RelativePosition)>,
}

impl Cell {
    pub fn is_alive(&self) -> bool {
        return match self.state {
            CellState::ALIVE => { true }
            CellState::DEAD => { false }
        }
    }

    pub fn tick(&self) {

    }

    pub fn new(state: CellState) -> Cell {
        Cell {
            state,
            neighbours: vec![]
        }
    }

    pub fn new_alive() -> Cell {
        Cell {
            state: CellState::ALIVE,
            neighbours: vec![]
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