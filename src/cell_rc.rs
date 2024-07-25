use std::rc::Rc;
use crate::cell::{CellState, RelativePosition};

#[derive(Debug)]
pub struct RcCell {
    state: CellState,
    neighbours: Vec<(Rc<RcCell>, RelativePosition)>, // TODO faut peut être mieux passer à un Rc en fait. A essayer
}

impl RcCell {
    pub fn is_alive(&self) -> bool {
        return match self.state {
            CellState::ALIVE => { true }
            CellState::DEAD => { false }
        };
    }

    pub fn add_neighbour(&mut self, neighbour: Rc<RcCell>, position: RelativePosition) {
        if (!self.has_neighbour_at_position(&position)) {
            self.neighbours.push((neighbour, position));
        }
    }

    pub fn number_of_neighbours(&self) -> usize {
        self.neighbours.len()
    }

    pub fn tick(&self) {
        let res: Vec<bool> = self.neighbours.iter().map(|(cell, position)| cell.is_alive()).collect();
        println!("Res in cell {:?}", res);
    }

    pub fn new(state: CellState) -> RcCell {
        RcCell {
            state,
            neighbours: vec![],
        }
    }

    pub fn new_alive() -> RcCell {
        RcCell {
            state: CellState::ALIVE,
            neighbours: vec![],
        }
    }

    pub fn new_dead() -> RcCell {
        RcCell {
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
    use crate::cell::Cell;
    use super::*;

    #[test]
    fn should_add_neighbour_to_cell() {
        let mut cell = RcCell::new_alive();
        let neighbour = RcCell::new_alive();
        let ref_neighbour = Rc::new(neighbour);

        cell.add_neighbour(Rc::clone(&ref_neighbour), RelativePosition::NORTH);

        assert_eq!(cell.number_of_neighbours(), 1);
    }

    // #[test]
    // fn should_be_alive_at_next_tick_when_alive() {
    //     let cell = Cell::new_alive();
    //
    //     cell.tick();
    //
    //     assert_eq!(cell.is_alive(), true);
    // }

    // #[test]
    // fn should_not_be_able_to_add_two_neighbours_at_same_position() {
    //     let mut cell = Cell::new_alive();
    //     let neighbour_one = Cell::new_alive();
    //     let neighbour_two = Cell::new_alive();
    //
    //     cell.add_neighbour(&neighbour_one, RelativePosition::EAST);
    //     cell.add_neighbour(&neighbour_one, RelativePosition::EAST);
    //
    //     let east_neighbours: usize = cell
    //         .neighbours
    //         .into_iter()
    //         .filter(|(neighbour, position)| match position {
    //             RelativePosition::NORTH |
    //             RelativePosition::WEST |
    //             RelativePosition::SOUTH => { false }
    //             RelativePosition::EAST => { true }
    //         })
    //         .map(|(cell, position)| cell)
    //         .count();
    //     assert_eq!(east_neighbours, 1);
    // }
    //
    // #[test]
    // fn should_be_alive_when_have_one_neighbour_alive_at_next_tick() {
    //     let (mut central, north, east, south, west) = generate_cell_with_neighbours(
    //         CellState::ALIVE,
    //         CellState::ALIVE,
    //         CellState::ALIVE,
    //         CellState::ALIVE,
    //     );
    //     central.add_neighbour(&north, RelativePosition::NORTH);
    //     central.add_neighbour(&east, RelativePosition::EAST);
    //     central.add_neighbour(&south, RelativePosition::SOUTH);
    //     central.add_neighbour(&east, RelativePosition::WEST);
    //
    //     central.tick();
    //
    //     assert_eq!(central.is_alive(), true);
    // }
    //
    // fn generate_cell_with_neighbours(
    //     northState: CellState,
    //     eastState: CellState,
    //     southState: CellState,
    //     westState: CellState,
    // ) -> (Cell, Cell, Cell, Cell, Cell) {
    //     let north = Cell::new(northState);
    //     let east = Cell::new(eastState);
    //     let south = Cell::new(southState);
    //     let west = Cell::new(westState);
    //     let mut central = Cell::new_alive();
    //
    //     (central, north, east, south, west)
    // }
}