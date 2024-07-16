use std::rc::Rc;

pub enum CellState {
    ALIVE,
    DEAD,
}

pub enum RelativePosition {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

pub struct Cell<'a> {
    state: CellState,
    neighbours: Vec<(&'a Cell<'a>, RelativePosition)>,
}

impl<'a> Cell<'a> {
    pub fn is_alive(&self) -> bool {
        return match self.state {
            CellState::ALIVE => { true }
            CellState::DEAD => { false }
        };
    }

    pub fn add_neighbour(&mut self, neighbour: &'a Cell, position: RelativePosition) {
        self.neighbours.push((neighbour, position));
    }

    pub fn tick(&self) {
        let res = self.neighbours.iter().all(|(cell, position)| cell.is_alive());
        println!("Res in cell {}", res);
    }

    pub fn new(state: CellState) -> Cell<'a> {
        Cell {
            state,
            neighbours: vec![],
        }
    }

    pub fn new_alive() -> Cell<'a> {
        Cell {
            state: CellState::ALIVE,
            neighbours: vec![],
        }
    }

    pub fn new_dead() -> Cell<'a> {
        Cell {
            state: CellState::DEAD,
            neighbours: vec![],
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

    #[test]
    fn should_be_alive_when_have_one_neighbour_alive_at_next_tick() {
        let mut central_cell = Cell::new_alive();
        let north_cell = Cell::new_alive();
        central_cell.add_neighbour(&north_cell, RelativePosition::NORTH);

        central_cell.tick();

        assert_eq!(central_cell.is_alive(), true);
    }
}