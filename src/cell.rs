use std::rc::Rc;

#[derive(Debug)]
pub enum CellState {
    ALIVE,
    DEAD,
}

#[derive(Debug, PartialEq)]
pub enum RelativePosition {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

#[derive(Debug)]
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
        if (!self.has_neighbour_at_position(&position)) {
            self.neighbours.push((neighbour, position));
        }
    }

    pub fn tick(&self) {
        let res: Vec<bool> = self.neighbours.iter().map(|(cell, position)| cell.is_alive()).collect();
        println!("Res in cell {:?}", res);
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

    fn has_neighbour_at_position(&self, requested_position: &RelativePosition) -> bool {
        return self.neighbours.iter().any(|(cell, position)| position == requested_position)
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
    fn should_not_be_able_to_add_two_neighbours_at_same_position<'a>() {
        let mut cell = Cell::new_alive();
        let neighbour_one = Cell::new_alive();
        let neighbour_two = Cell::new_alive();

        cell.add_neighbour(&neighbour_one, RelativePosition::EAST);
        cell.add_neighbour(&neighbour_one, RelativePosition::EAST);

        let east_neighbours: usize = cell
            .neighbours
            .into_iter()
            .filter(|(neighbour, position)| match position {
                RelativePosition::NORTH |
                RelativePosition::WEST |
                RelativePosition::SOUTH => { false }
                RelativePosition::EAST => { true }
            })
            .map(|(cell, position)| cell)
            .count();
        assert_eq!(east_neighbours, 1);
    }

    #[test]
    fn should_be_alive_when_have_one_neighbour_alive_at_next_tick() {
        let mut central_cell = Cell::new_alive();
        let north_cell = Cell::new_alive();
        central_cell.add_neighbour(&north_cell, RelativePosition::NORTH);

        central_cell.tick();

        // assert_eq!(central_cell.is_alive(), true);
    }
}