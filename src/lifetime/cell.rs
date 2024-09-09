use crate::common::cell_state::CellState;
use crate::common::relative_position::RelativePosition;

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
        if !self.has_neighbour_at_position(&position) {
            self.neighbours.push((neighbour, position));
        }
    }

    pub fn tick(&self) -> CellState {
        let number_of_live_neighbours = self.count_live_neighbours();
        match number_of_live_neighbours {
            n if n < 2 || n > 3 => {
                CellState::DEAD
            }
            3 => {
                CellState::ALIVE
            }
            _ => {
                match self.state {
                    CellState::ALIVE => CellState::ALIVE,
                    CellState::DEAD => CellState::DEAD,
                }
            }
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

    fn count_live_neighbours(&self) -> usize {
        self
            .neighbours
            .iter()
            .map(|(cell, _)| cell.is_alive())
            .filter(|x| *x == true)
            .count()
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

        cell.add_neighbour(&neighbour_one, RelativePosition::East);
        cell.add_neighbour(&neighbour_one, RelativePosition::East);

        let east_neighbours: usize = cell
            .neighbours
            .into_iter()
            .filter(|(neighbour, position)| match position {
                RelativePosition::East => { true }
                _ => { false }
            })
            .map(|(cell, position)| cell)
            .count();
        assert_eq!(east_neighbours, 1);
    }

    mod game_rules {
        use crate::common::cell_state::CellState;
        use crate::common::relative_position::RelativePosition;
        use crate::lifetime::cell::Cell;

        // Any live cell with fewer than two live neighbours dies, as if caused by under-population.
        #[test]
        fn should_be_dead_when_have_one_neighbour_alive_at_next_tick() {
            let north = Cell::new_alive();
            let north_est = Cell::new_dead();
            let east = Cell::new_dead();
            let south_east = Cell::new_dead();
            let south = Cell::new_dead();
            let south_west = Cell::new_dead();
            let west = Cell::new_dead();
            let north_west = Cell::new_dead();
            let mut central = Cell::new_alive();
            central.add_neighbour(&north, RelativePosition::North);
            central.add_neighbour(&north_est, RelativePosition::NorthEast);
            central.add_neighbour(&east, RelativePosition::East);
            central.add_neighbour(&south_east, RelativePosition::SouthEast);
            central.add_neighbour(&south, RelativePosition::South);
            central.add_neighbour(&south_west, RelativePosition::SouthWest);
            central.add_neighbour(&west, RelativePosition::West);
            central.add_neighbour(&north_west, RelativePosition::NorthWest);

            let new_state = central.tick();

            assert_eq!(new_state, CellState::DEAD);
        }

        // Any live cell with two or three live neighbours lives on to the next generation.
        #[test]
        fn should_be_alive_when_have_two_or_three_neighbours_alive_at_next_tick() {
            let north = Cell::new_alive();
            let north_est = Cell::new_alive();
            let east = Cell::new_alive();
            let south_east = Cell::new_dead();
            let south = Cell::new_dead();
            let south_west = Cell::new_dead();
            let west = Cell::new_dead();
            let north_west = Cell::new_dead();
            let mut central = Cell::new_alive();
            central.add_neighbour(&north, RelativePosition::North);
            central.add_neighbour(&north_est, RelativePosition::NorthEast);
            central.add_neighbour(&east, RelativePosition::East);
            central.add_neighbour(&south_east, RelativePosition::SouthEast);
            central.add_neighbour(&south, RelativePosition::South);
            central.add_neighbour(&south_west, RelativePosition::SouthWest);
            central.add_neighbour(&west, RelativePosition::West);
            central.add_neighbour(&north_west, RelativePosition::NorthWest);

            let next_state = central.tick();

            assert_eq!(next_state, CellState::ALIVE);
        }

        // Any live cell with more than three live neighbours dies, as if by overcrowding.
        #[test]
        fn should_be_dead_when_more_then_three_neighbours_alive_at_next_tick() {
            let north = Cell::new_alive();
            let north_est = Cell::new_alive();
            let east = Cell::new_alive();
            let south_east = Cell::new_alive();
            let south = Cell::new_dead();
            let south_west = Cell::new_dead();
            let west = Cell::new_dead();
            let north_west = Cell::new_dead();
            let mut central = Cell::new_alive();
            central.add_neighbour(&north, RelativePosition::North);
            central.add_neighbour(&north_est, RelativePosition::NorthEast);
            central.add_neighbour(&east, RelativePosition::East);
            central.add_neighbour(&south_east, RelativePosition::SouthEast);
            central.add_neighbour(&south, RelativePosition::South);
            central.add_neighbour(&south_west, RelativePosition::SouthWest);
            central.add_neighbour(&west, RelativePosition::West);
            central.add_neighbour(&north_west, RelativePosition::NorthWest);

            let next_state = central.tick();

            assert_eq!(next_state, CellState::DEAD);
        }

        // Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
        #[test]
        fn should_be_alive_when_three_live_neighbours_alive_at_next_tick() {
            let north = Cell::new_alive();
            let north_est = Cell::new_alive();
            let east = Cell::new_alive();
            let south_east = Cell::new_dead();
            let south = Cell::new_dead();
            let south_west = Cell::new_dead();
            let west = Cell::new_dead();
            let north_west = Cell::new_dead();
            let mut central = Cell::new_dead();
            central.add_neighbour(&north, RelativePosition::North);
            central.add_neighbour(&north_est, RelativePosition::NorthEast);
            central.add_neighbour(&east, RelativePosition::East);
            central.add_neighbour(&south_east, RelativePosition::SouthEast);
            central.add_neighbour(&south, RelativePosition::South);
            central.add_neighbour(&south_west, RelativePosition::SouthWest);
            central.add_neighbour(&west, RelativePosition::West);
            central.add_neighbour(&north_west, RelativePosition::NorthWest);

            let next_state = central.tick();

            assert_eq!(next_state, CellState::ALIVE);
        }
    }
}