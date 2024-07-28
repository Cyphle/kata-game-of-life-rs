use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum CellState {
    ALIVE,
    DEAD,
}

#[derive(Debug, PartialEq)]
pub enum RelativePosition {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

#[derive(Debug, PartialEq)]
pub struct Cell {
    state: CellState,
    neighbours: Vec<(Rc<Cell>, RelativePosition)>, // TODO faut peut être mieux passer à un Rc en fait. A essayer
}

impl Cell {
    pub fn is_alive(&self) -> bool {
        match self.state {
            CellState::ALIVE => { true }
            CellState::DEAD => { false }
        }
    }
    pub fn is_dead(&self) -> bool {
        !self.is_alive()
    }

    pub fn add_neighbour(&mut self, neighbour: Rc<Cell>, position: RelativePosition) {
        if !self.has_neighbour_at_position(&position) {
            self.neighbours.push((neighbour, position));
        }
    }

    pub fn number_of_neighbours(&self) -> usize {
        self.neighbours.len()
    }

    // TODO en fait le tick doit renvoyer une nouvelle version au lieu de mut
    pub fn tick(&self) -> Cell {
        let copy_of_neighbours = self.neighbours
            .iter()
            .map(|(n, position)| (
                Rc::clone(n),
                match position {
                    RelativePosition::North => RelativePosition::North,
                    RelativePosition::NorthEast => RelativePosition::NorthEast,
                    RelativePosition::East => RelativePosition::East,
                    RelativePosition::SouthEast => RelativePosition::SouthEast,
                    RelativePosition::South => RelativePosition::South,
                    RelativePosition::SouthWest => RelativePosition::SouthWest,
                    RelativePosition::West => RelativePosition::West,
                    RelativePosition::NorthWest => RelativePosition::NorthWest,
                }
            ))
            .collect();

        let number_of_live_neighbours = self.count_live_neighbours();
        match number_of_live_neighbours {
            n if n < 2 || n > 3 => {
                Cell {
                    state: CellState::DEAD,
                    neighbours: copy_of_neighbours,
                }
            }
            3 => {
                Cell {
                    state: CellState::ALIVE,
                    neighbours: copy_of_neighbours,
                }
            }
            _ => {
                Cell {
                    state: match self.state {
                        CellState::ALIVE => CellState::ALIVE,
                        CellState::DEAD => CellState::DEAD,
                    },
                    neighbours: copy_of_neighbours,
                }
            }
        }
    }

    pub fn new(state: CellState) -> Cell {
        Cell {
            state,
            neighbours: vec![],
        }
    }

    pub fn new_alive() -> Cell {
        Cell {
            state: CellState::ALIVE,
            neighbours: vec![],
        }
    }

    pub fn new_dead() -> Cell {
        Cell {
            state: CellState::DEAD,
            neighbours: vec![],
        }
    }

    pub fn print(&self) -> String {
        match self.is_alive() {
            true => { "x".to_string() }
            false => { "o".to_string() }
        }
    }

    fn count_live_neighbours(&self) -> usize {
        self
            .neighbours
            .iter()
            .map(|(cell, _)| cell.is_alive())
            .filter(|x| *x == true)
            .count()
    }

    fn has_neighbour_at_position(&self, requested_position: &RelativePosition) -> bool {
        return self.neighbours.iter().any(|(_, position)| position == requested_position);
    }
}

#[cfg(test)]
mod cell_tests {
    use super::*;

    #[test]
    fn should_add_neighbour_to_cell() {
        let mut cell = Cell::new_alive();
        let neighbour = Cell::new_alive();
        let ref_neighbour = Rc::new(neighbour);

        cell.add_neighbour(Rc::clone(&ref_neighbour), RelativePosition::North);

        assert_eq!(cell.number_of_neighbours(), 1);
    }

    #[test]
    fn should_be_alive_at_next_tick_when_alive() {
        let mut cell = Cell::new_alive();

        cell.tick();

        assert_eq!(cell.is_alive(), true);
    }

    #[test]
    fn should_not_be_able_to_add_two_neighbours_at_same_position() {
        let mut cell = Cell::new_alive();
        let neighbour_one = Rc::new(Cell::new_alive());
        let neighbour_two = Rc::new(Cell::new_alive());

        cell.add_neighbour(Rc::clone(&neighbour_one), RelativePosition::East);
        cell.add_neighbour(Rc::clone(&neighbour_two), RelativePosition::East);

        let east_neighbours: usize = cell
            .neighbours
            .into_iter()
            .filter(|(_, position)| match position {
                RelativePosition::East => { true }
                _ => { false }
            })
            .map(|(cell, _)| cell)
            .count();
        assert_eq!(east_neighbours, 1);
    }

    mod game_rules {
        use std::cell::RefCell;
        use std::rc::Rc;
        use crate::cell::{Cell, RelativePosition};

        // Any live cell with fewer than two live neighbours dies, as if caused by under-population.
        #[test]
        fn should_be_dead_when_have_one_neighbour_alive_at_next_tick() {
            let north = Rc::new(Cell::new_alive());
            let north_est = Rc::new(Cell::new_dead());
            let east = Rc::new(Cell::new_dead());
            let south_east = Rc::new(Cell::new_dead());
            let south = Rc::new(Cell::new_dead());
            let south_west = Rc::new(Cell::new_dead());
            let west = Rc::new(Cell::new_dead());
            let north_west = Rc::new(Cell::new_dead());
            let mut central = Rc::new(RefCell::new(Cell::new_alive()));
            central.borrow_mut().add_neighbour(Rc::clone(&north), RelativePosition::North);
            central.borrow_mut().add_neighbour(Rc::clone(&north_est), RelativePosition::NorthEast);
            central.borrow_mut().add_neighbour(Rc::clone(&east), RelativePosition::East);
            central.borrow_mut().add_neighbour(Rc::clone(&south_east), RelativePosition::SouthEast);
            central.borrow_mut().add_neighbour(Rc::clone(&south), RelativePosition::South);
            central.borrow_mut().add_neighbour(Rc::clone(&south_west), RelativePosition::SouthWest);
            central.borrow_mut().add_neighbour(Rc::clone(&west), RelativePosition::West);
            central.borrow_mut().add_neighbour(Rc::clone(&north_west), RelativePosition::NorthWest);

            let new_cell = central.borrow_mut().tick();

            assert_eq!(new_cell.is_alive(), false);
        }

        // Any live cell with two or three live neighbours lives on to the next generation.
        #[test]
        fn should_be_alive_when_have_two_or_three_neighbours_alive_at_next_tick() {
            let north = Rc::new(Cell::new_alive());
            let north_est = Rc::new(Cell::new_alive());
            let east = Rc::new(Cell::new_alive());
            let south_east = Rc::new(Cell::new_dead());
            let south = Rc::new(Cell::new_dead());
            let south_west = Rc::new(Cell::new_dead());
            let west = Rc::new(Cell::new_dead());
            let north_west = Rc::new(Cell::new_dead());
            let mut central = Cell::new_alive();
            central.add_neighbour(Rc::clone(&north), RelativePosition::North);
            central.add_neighbour(Rc::clone(&north_est), RelativePosition::NorthEast);
            central.add_neighbour(Rc::clone(&east), RelativePosition::East);
            central.add_neighbour(Rc::clone(&south_east), RelativePosition::SouthEast);
            central.add_neighbour(Rc::clone(&south), RelativePosition::South);
            central.add_neighbour(Rc::clone(&south_west), RelativePosition::SouthWest);
            central.add_neighbour(Rc::clone(&west), RelativePosition::West);
            central.add_neighbour(Rc::clone(&north_west), RelativePosition::NorthWest);

            let new_cell = central.tick();

            assert_eq!(new_cell.is_alive(), true);
        }

        // Any live cell with more than three live neighbours dies, as if by overcrowding.
        #[test]
        fn should_be_dead_when_more_then_three_neighbours_alive_at_next_tick() {
            let north = Rc::new(Cell::new_alive());
            let north_est = Rc::new(Cell::new_alive());
            let east = Rc::new(Cell::new_alive());
            let south_east = Rc::new(Cell::new_alive());
            let south = Rc::new(Cell::new_dead());
            let south_west = Rc::new(Cell::new_dead());
            let west = Rc::new(Cell::new_dead());
            let north_west = Rc::new(Cell::new_dead());
            let mut central = Cell::new_alive();
            central.add_neighbour(Rc::clone(&north), RelativePosition::North);
            central.add_neighbour(Rc::clone(&north_est), RelativePosition::NorthEast);
            central.add_neighbour(Rc::clone(&east), RelativePosition::East);
            central.add_neighbour(Rc::clone(&south_east), RelativePosition::SouthEast);
            central.add_neighbour(Rc::clone(&south), RelativePosition::South);
            central.add_neighbour(Rc::clone(&south_west), RelativePosition::SouthWest);
            central.add_neighbour(Rc::clone(&west), RelativePosition::West);
            central.add_neighbour(Rc::clone(&north_west), RelativePosition::NorthWest);

            let new_cell = central.tick();

            assert_eq!(new_cell.is_alive(), false);
        }

        // Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
        #[test]
        fn should_be_alive_when_three_live_neighbours_alive_at_next_tick() {
            let north = Rc::new(Cell::new_alive());
            let north_est = Rc::new(Cell::new_alive());
            let east = Rc::new(Cell::new_alive());
            let south_east = Rc::new(Cell::new_dead());
            let south = Rc::new(Cell::new_dead());
            let south_west = Rc::new(Cell::new_dead());
            let west = Rc::new(Cell::new_dead());
            let north_west = Rc::new(Cell::new_dead());
            let mut central = Cell::new_dead();
            central.add_neighbour(Rc::clone(&north), RelativePosition::North);
            central.add_neighbour(Rc::clone(&north_est), RelativePosition::NorthEast);
            central.add_neighbour(Rc::clone(&east), RelativePosition::East);
            central.add_neighbour(Rc::clone(&south_east), RelativePosition::SouthEast);
            central.add_neighbour(Rc::clone(&south), RelativePosition::South);
            central.add_neighbour(Rc::clone(&south_west), RelativePosition::SouthWest);
            central.add_neighbour(Rc::clone(&west), RelativePosition::West);
            central.add_neighbour(Rc::clone(&north_west), RelativePosition::NorthWest);

            let new_cell = central.tick();

            assert_eq!(new_cell.is_alive(), true);
        }
    }
}