use std::cell::RefCell;
use std::rc::Rc;
use rand::Rng;
use crate::common::cell_state::CellState;
use crate::common::relative_position::RelativePosition;

#[derive(Debug, PartialEq)]
pub struct Cell {
    state: CellState,
    next_state: CellState,
    neighbours: Vec<(Rc<RefCell<Cell>>, RelativePosition)>,
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

    pub fn add_neighbour(&mut self, neighbour: Rc<RefCell<Cell>>, position: RelativePosition) {
        if !self.has_neighbour_at_position(&position) {
            self.neighbours.push((neighbour, position));
        }
    }

    pub fn number_of_neighbours(&self) -> usize {
        self.neighbours.len()
    }

    pub fn pretick(&mut self) {
        let number_of_live_neighbours = self.count_live_neighbours();
        match number_of_live_neighbours {
            n if n < 2 || n > 3 => {
                self.next_state = CellState::DEAD
            }
            3 => {
                self.next_state = CellState::ALIVE
            }
            _ => {
                self.next_state = match self.state {
                    CellState::ALIVE => CellState::ALIVE,
                    CellState::DEAD => CellState::DEAD,
                }
            }
        }
    }

    pub fn tick(&mut self) {
        self.state = match self.next_state {
            CellState::ALIVE => CellState::ALIVE,
            CellState::DEAD => CellState::DEAD
        }
    }

    pub fn new(state: &CellState) -> Cell {
        Cell {
            state: match state {
                CellState::ALIVE => CellState::ALIVE,
                CellState::DEAD => CellState::DEAD
            },
            next_state: CellState::ALIVE,
            neighbours: vec![],
        }
    }

    pub fn new_random_state() -> Cell {
        let state = rand::thread_rng().gen_range(0..2);
        Cell {
            state: match state {
                0 => CellState::DEAD,
                _ => CellState::ALIVE,
            },
            next_state: CellState::ALIVE,
            neighbours: vec![],
        }
    }

    pub fn new_alive() -> Cell {
        Cell {
            state: CellState::ALIVE,
            next_state: CellState::ALIVE,
            neighbours: vec![],
        }
    }

    pub fn new_dead() -> Cell {
        Cell {
            state: CellState::DEAD,
            next_state: CellState::ALIVE,
            neighbours: vec![],
        }
    }

    pub fn print(&self) -> String {
        let print = match self.is_alive() {
            true => { "x".to_string() }
            false => { "o".to_string() }
        };
        format!("{}", print)
    }

    pub fn print_neighbours_count(&self) -> String {
        return format!("({}n)", self.neighbours.len());
    }

    pub fn print_neighbours_positions(&self) -> String {
        self.neighbours
            .iter()
            .map(|(cell, position)| position.print())
            .collect::<Vec<String>>()
            .join(",")
    }

    fn count_live_neighbours(&self) -> usize {
        self
            .neighbours
            .iter()
            .map(|(cell, _)| cell.borrow().is_alive())
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
        let ref_neighbour = Rc::new(RefCell::new(neighbour));

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
        let neighbour_one = Rc::new(RefCell::new(Cell::new_alive()));
        let neighbour_two = Rc::new(RefCell::new(Cell::new_alive()));

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
        use crate::common::relative_position::RelativePosition;
        use crate::smartpointers::cell::Cell;

        // Any live cell with fewer than two live neighbours dies, as if caused by under-population.
        #[test]
        fn should_be_dead_when_have_one_neighbour_alive_at_next_tick() {
            let north = Rc::new(RefCell::new(Cell::new_alive()));
            let north_est = Rc::new(RefCell::new(Cell::new_dead()));
            let east = Rc::new(RefCell::new(Cell::new_dead()));
            let south_east = Rc::new(RefCell::new(Cell::new_dead()));
            let south = Rc::new(RefCell::new(Cell::new_dead()));
            let south_west = Rc::new(RefCell::new(Cell::new_dead()));
            let west = Rc::new(RefCell::new(Cell::new_dead()));
            let north_west = Rc::new(RefCell::new(Cell::new_dead()));
            let central = Rc::new(RefCell::new(Cell::new_alive()));
            central.borrow_mut().add_neighbour(Rc::clone(&north), RelativePosition::North);
            central.borrow_mut().add_neighbour(Rc::clone(&north_est), RelativePosition::NorthEast);
            central.borrow_mut().add_neighbour(Rc::clone(&east), RelativePosition::East);
            central.borrow_mut().add_neighbour(Rc::clone(&south_east), RelativePosition::SouthEast);
            central.borrow_mut().add_neighbour(Rc::clone(&south), RelativePosition::South);
            central.borrow_mut().add_neighbour(Rc::clone(&south_west), RelativePosition::SouthWest);
            central.borrow_mut().add_neighbour(Rc::clone(&west), RelativePosition::West);
            central.borrow_mut().add_neighbour(Rc::clone(&north_west), RelativePosition::NorthWest);

            central.borrow_mut().pretick();
            central.borrow_mut().tick();

            assert_eq!(central.borrow().is_alive(), false);
        }

        // Any live cell with two or three live neighbours lives on to the next generation.
        #[test]
        fn should_be_alive_when_have_two_or_three_neighbours_alive_at_next_tick() {
            let north = Rc::new(RefCell::new(Cell::new_alive()));
            let north_est = Rc::new(RefCell::new(Cell::new_alive()));
            let east = Rc::new(RefCell::new(Cell::new_alive()));
            let south_east = Rc::new(RefCell::new(Cell::new_dead()));
            let south = Rc::new(RefCell::new(Cell::new_dead()));
            let south_west = Rc::new(RefCell::new(Cell::new_dead()));
            let west = Rc::new(RefCell::new(Cell::new_dead()));
            let north_west = Rc::new(RefCell::new(Cell::new_dead()));
            let mut central = Cell::new_alive();
            central.add_neighbour(Rc::clone(&north), RelativePosition::North);
            central.add_neighbour(Rc::clone(&north_est), RelativePosition::NorthEast);
            central.add_neighbour(Rc::clone(&east), RelativePosition::East);
            central.add_neighbour(Rc::clone(&south_east), RelativePosition::SouthEast);
            central.add_neighbour(Rc::clone(&south), RelativePosition::South);
            central.add_neighbour(Rc::clone(&south_west), RelativePosition::SouthWest);
            central.add_neighbour(Rc::clone(&west), RelativePosition::West);
            central.add_neighbour(Rc::clone(&north_west), RelativePosition::NorthWest);

            central.pretick();
            central.tick();

            assert_eq!(central.is_alive(), true);
        }

        // Any live cell with more than three live neighbours dies, as if by overcrowding.
        #[test]
        fn should_be_dead_when_more_then_three_neighbours_alive_at_next_tick() {
            let north = Rc::new(RefCell::new(Cell::new_alive()));
            let north_est = Rc::new(RefCell::new(Cell::new_alive()));
            let east = Rc::new(RefCell::new(Cell::new_alive()));
            let south_east = Rc::new(RefCell::new(Cell::new_alive()));
            let south = Rc::new(RefCell::new(Cell::new_dead()));
            let south_west = Rc::new(RefCell::new(Cell::new_dead()));
            let west = Rc::new(RefCell::new(Cell::new_dead()));
            let north_west = Rc::new(RefCell::new(Cell::new_dead()));
            let mut central = Cell::new_alive();
            central.add_neighbour(Rc::clone(&north), RelativePosition::North);
            central.add_neighbour(Rc::clone(&north_est), RelativePosition::NorthEast);
            central.add_neighbour(Rc::clone(&east), RelativePosition::East);
            central.add_neighbour(Rc::clone(&south_east), RelativePosition::SouthEast);
            central.add_neighbour(Rc::clone(&south), RelativePosition::South);
            central.add_neighbour(Rc::clone(&south_west), RelativePosition::SouthWest);
            central.add_neighbour(Rc::clone(&west), RelativePosition::West);
            central.add_neighbour(Rc::clone(&north_west), RelativePosition::NorthWest);

            central.pretick();
            central.tick();

            assert_eq!(central.is_alive(), false);
        }

        // Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
        #[test]
        fn should_be_alive_when_three_live_neighbours_alive_at_next_tick() {
            let north = Rc::new(RefCell::new(Cell::new_alive()));
            let north_est = Rc::new(RefCell::new(Cell::new_alive()));
            let east = Rc::new(RefCell::new(Cell::new_alive()));
            let south_east = Rc::new(RefCell::new(Cell::new_dead()));
            let south = Rc::new(RefCell::new(Cell::new_dead()));
            let south_west = Rc::new(RefCell::new(Cell::new_dead()));
            let west = Rc::new(RefCell::new(Cell::new_dead()));
            let north_west = Rc::new(RefCell::new(Cell::new_dead()));
            let mut central = Cell::new_dead();
            central.add_neighbour(Rc::clone(&north), RelativePosition::North);
            central.add_neighbour(Rc::clone(&north_est), RelativePosition::NorthEast);
            central.add_neighbour(Rc::clone(&east), RelativePosition::East);
            central.add_neighbour(Rc::clone(&south_east), RelativePosition::SouthEast);
            central.add_neighbour(Rc::clone(&south), RelativePosition::South);
            central.add_neighbour(Rc::clone(&south_west), RelativePosition::SouthWest);
            central.add_neighbour(Rc::clone(&west), RelativePosition::West);
            central.add_neighbour(Rc::clone(&north_west), RelativePosition::NorthWest);

            central.pretick();
            central.tick();

            assert_eq!(central.is_alive(), true);
        }
    }
}