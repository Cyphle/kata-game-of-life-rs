use rand::Rng;

use crate::common::cell_state::CellState;
use crate::common::relative_position::RelativePosition;
use crate::nopointer::cell::Cell;

static UNIVERSE_START_INDEX: usize = 0;

#[derive(Debug)]
struct CellPosition {
    x: usize,
    y: usize,
    cell: Cell,
}

#[derive(Debug)]
pub struct Universe {
    width: usize,
    height: usize,
    cells: Vec<Vec<CellPosition>>,
}

impl Universe {
    /*
        INSTANCE
     */
    pub fn tick(&self) -> Universe {
        let mut new_cells: Vec<Vec<CellPosition>> = vec![];

        for y in UNIVERSE_START_INDEX..self.height {
            let mut line: Vec<CellPosition> = vec![];
            for x in UNIVERSE_START_INDEX..self.width {
                let cell = match self.cells.get(y) {
                    Some(line_of_cells) => {
                        match line_of_cells.get(x) {
                            None => Cell::new_random_state(),
                            Some(cell_position) => {
                                let neighbours_states = self.get_neighbours_states_of(x, y);
                                Cell::new(&self.next_state_of(cell_position.cell.get_state(), neighbours_states))
                            }
                        }
                    }
                    _ => {
                        Cell::new_random_state()
                    }
                };

                line.push(CellPosition {
                    x,
                    y,
                    cell,
                });
            }

            new_cells.push(line);
        }

        Universe {
            width: self.width,
            height: self.height,
            cells: new_cells,
        }
    }

    fn next_state_of(&self, cell_state: CellState, neighbour_state: Vec<CellState>) -> CellState {
        let alive_neighbours_count = neighbour_state
            .iter()
            .filter(|state| state == &&CellState::ALIVE)
            .count();

        match alive_neighbours_count {
            n if n < 2 || n > 3 => {
                CellState::DEAD
            }
            3 => {
                CellState::ALIVE
            }
            _ => {
                cell_state.clone()
            }
        }
    }

    fn get_neighbours_states_of(&self, x: usize, y: usize) -> Vec<CellState> {
        let column_neighbours_start = if y > 0 { y - 1 } else { 0 };
        let column_neighbours_end = if y + 1 < self.height { y + 1 } else { self.height - 1 };

        (column_neighbours_start..=column_neighbours_end)
            .fold(vec![], |mut acc, column_index| {
                let line_neighbours_start = if x > 0 { x - 1 } else { 0 };
                let line_neighbours_end = if x + 1 < self.width { x + 1 } else { self.width - 1 };

                (line_neighbours_start..=line_neighbours_end)
                    .for_each(|line_index| {
                        match self.cells.get(column_index) {
                            None => {}
                            Some(column) => {
                                match column.get(line_index) {
                                    None => {}
                                    Some(neighbour) => {
                                        if !(column_index == y && line_index == x) {
                                            acc.push(neighbour.cell.get_state());
                                        }
                                    }
                                }
                            }
                        }
                    });

                acc
            })
            .into_iter()                    // Iterate over the outer vector
            .collect::<Vec<CellState>>()        // Collect it back into a Vec<String>
    }

    fn count_neighbours_of(&self, x: usize, y: usize) -> usize {
        let column_neighbours_start = if y > 0 { y - 1 } else { 0 };
        let column_neighbours_end = if y + 1 < self.height { y + 1 } else { self.height - 1 };

        (column_neighbours_start..=column_neighbours_end)
            .fold(0, |acc_columns, column_index| {
                let line_neighbours_start = if x > 0 { x - 1 } else { 0 };
                let line_neighbours_end = if x + 1 < self.width { x + 1 } else { self.width - 1 };

                acc_columns + (line_neighbours_start..=line_neighbours_end)
                    .fold(0, |acc_lines, line_index| {
                        match self.cells.get(column_index) {
                            None => acc_lines,
                            Some(column) => {
                                match column.get(line_index) {
                                    None => acc_lines,
                                    Some(_) => {
                                        if !(column_index == y && line_index == x) {
                                            acc_lines + 1
                                        } else {
                                            acc_lines
                                        }
                                    }
                                }
                            }
                        }
                    })
            })
    }

    fn neighbours_positions_of(&self, x: usize, y: usize) -> String {
        let column_neighbours_start = if y > 0 { y - 1 } else { 0 };
        let column_neighbours_end = if y + 1 < self.height { y + 1 } else { self.height - 1 };

        (column_neighbours_start..=column_neighbours_end)
            .fold(vec![], |mut acc_columns, column_index| {
                let line_neighbours_start = if x > 0 { x - 1 } else { 0 };
                let line_neighbours_end = if x + 1 < self.width { x + 1 } else { self.width - 1 };

                acc_columns.push((line_neighbours_start..=line_neighbours_end)
                    .fold(vec![], |mut acc_lines, line_index| {
                        match self.cells.get(column_index) {
                            None => acc_lines,
                            Some(column) => {
                                match column.get(line_index) {
                                    None => acc_lines,
                                    Some(_) => {
                                        if !(column_index == y && line_index == x) {
                                            acc_lines.push(RelativePosition::get_position_from(x, y, line_index, column_index).print());
                                        }
                                        acc_lines
                                    }
                                }
                            }
                        }
                    }));

                acc_columns
            })
            .into_iter()                    // Iterate over the outer vector
            .map(|inner| inner.join(","))    // Join each inner vector with spaces
            .collect::<Vec<String>>()        // Collect it back into a Vec<String>
            .join(",")
    }

    pub fn print(&self) -> Vec<String> {
        self
            .cells
            .iter()
            .map(|x| x
                .iter()
                .map(|y| y.cell.print())
                .collect::<Vec<String>>()
                .join(" ")
            )
            .collect()
    }

    fn print_check(&self) -> Vec<String> {
        self
            .cells
            .iter()
            .map(|x| x
                .iter()
                .map(|y| format!("({}{})({}:{})", y.y, y.x, self.count_neighbours_of(y.x, y.y), self.neighbours_positions_of(y.x, y.y)))
                .collect::<Vec<String>>()
                .join(" ")
            )
            .collect()
    }

    /*
        STATIC
     */
    pub fn new(width: usize, height: usize) -> Universe {
        let states = Self::generate_base_states(width, height);
        Universe::new_from_cell_states(states)
    }

    fn generate_base_states(width: usize, height: usize) -> Vec<Vec<CellState>> {
        let mut states: Vec<Vec<CellState>> = vec![];

        for _ in 0..height {
            let mut line_states: Vec<CellState> = vec![];
            for _ in 0..width {
                let rand = rand::thread_rng().gen_range(0..2);
                let state = match rand {
                    0 => CellState::DEAD,
                    _ => CellState::ALIVE,
                };
                line_states.push(state);
            }
            states.push(line_states);
        }

        states
    }

    pub fn new_from_states(states: &Vec<&str>) -> Universe {
        Self::new_from_cell_states(states
            .into_iter()
            .map(|line| {
                line
                    .chars()
                    .filter(|cell| cell != &' ')
                    .map(|cell| {
                        match cell {
                            'x' => CellState::ALIVE,
                            _ => CellState::DEAD
                        }
                    })
                    .collect::<Vec<CellState>>()
            })
            .collect::<Vec<Vec<CellState>>>())
    }

    fn new_from_cell_states(states: Vec<Vec<CellState>>) -> Universe {
        let height = states.len();
        let width = states[0].len();

        let mut cells: Vec<Vec<CellPosition>> = vec![];

        for y in UNIVERSE_START_INDEX..height {
            let mut line: Vec<CellPosition> = vec![];
            for x in UNIVERSE_START_INDEX..width {
                let mut cell = match states.get(y) {
                    Some(line_of_states) => {
                        match line_of_states.get(x) {
                            None => Cell::new_random_state(),
                            Some(state) => Cell::new(state)
                        }
                    }
                    _ => {
                        Cell::new_random_state()
                    }
                };

                line.push(CellPosition {
                    x,
                    y,
                    cell,
                });
            }

            cells.push(line);
        }

        Universe {
            width,
            height,
            cells,
        }
    }
}

#[cfg(test)]
mod universe_tests {
    use crate::common::cell_state::CellState;
    use crate::nopointer::universe::Universe;

    #[test]
    fn should_be_able_to_generate_a_monocellular_universe() {
        let universe = Universe::new(1, 1);

        for line_to_print in universe.print() {
            assert_eq!(line_to_print, "x");
            println!("{:?}", line_to_print);
        }
    }

    #[test]
    fn should_be_able_to_generate_a_linear_universe_of_two_cells() {
        let universe = Universe::new(2, 1);

        print_universe(&universe);
        for line_to_print in universe.print_check() {
            assert_eq!(line_to_print, "(00)(1:E) (01)(1:W)");
        }
    }

    #[test]
    fn should_be_able_to_generate_a_vertical_universe_of_two_cells() {
        let universe = Universe::new(1, 2);

        print_universe(&universe);
        let lines_to_print = universe.print_check();
        assert_eq!(lines_to_print[0], "(00)(1:S)");
        assert_eq!(lines_to_print[1], "(10)(1:N)");
    }

    #[test]
    fn should_be_able_to_generate_a_square_universe_of_two_cells() {
        let universe = Universe::new(2, 2);

        print_check_universe(&universe);
        let print_check = universe.print_check();
        assert_eq!(print_check[0], "(00)(3:E,S,SE) (01)(3:W,SW,S)");
        assert_eq!(print_check[1], "(10)(3:N,NE,E) (11)(3:NW,N,W)");
    }

    #[test]
    fn should_be_able_to_generate_a_linear_universe_of_three_cells_with_random_state() {
        let universe = Universe::new(3, 1);

        print_universe(&universe);
        for line_to_print in universe.print_check() {
            assert_eq!(line_to_print, "(00)(1:E) (01)(2:W,E) (02)(1:W)");
        }
    }

    #[test]
    fn should_be_able_to_generate_a_vertical_universe_of_three_cells_with_random_state() {
        let universe = Universe::new(3, 1);

        print_universe(&universe);
        let print_check = universe.print_check();
        assert_eq!(print_check[0], "(00)(1:E) (01)(2:W,E) (02)(1:W)");
    }

    #[test]
    fn should_be_able_to_generate_a_square_universe_of_three_cells() {
        let universe = Universe::new(3, 3);

        print_check_universe(&universe);
        let lines_to_print = universe.print_check();
        assert_eq!(lines_to_print[0], "(00)(3:E,S,SE) (01)(5:W,E,SW,S,SE) (02)(3:W,SW,S)");
        assert_eq!(lines_to_print[1], "(10)(5:N,NE,E,S,SE) (11)(8:NW,N,NE,W,E,SW,S,SE) (12)(5:NW,N,W,SW,S)");
        assert_eq!(lines_to_print[2], "(20)(3:N,NE,E) (21)(5:NW,N,NE,W,E) (22)(3:NW,N,W)");
    }

    #[test]
    fn should_be_able_to_generate_a_square_universe_of_three_cells_with_predefined_states() {
        let state = vec![
            vec![CellState::ALIVE, CellState::DEAD, CellState::ALIVE],
            vec![CellState::DEAD, CellState::ALIVE, CellState::DEAD],
            vec![CellState::ALIVE, CellState::DEAD, CellState::ALIVE],
        ];
        let universe = Universe::new_from_cell_states(state);

        print_check_universe(&universe);
        let lines_to_print = universe.print();
        assert_eq!(lines_to_print[0], "x o x");
        assert_eq!(lines_to_print[1], "o x o");
        assert_eq!(lines_to_print[2], "x o x");
    }

    #[test]
    fn should_be_able_to_generate_a_square_universe_of_three_cells_with_predefined_states_str() {
        let state = vec![
            "x o x",
            "o x o",
            "x o x"
        ];
        let universe = Universe::new_from_states(&state);

        print_check_universe(&universe);
        let lines_to_print = universe.print();
        assert_eq!(lines_to_print[0], "x o x");
        assert_eq!(lines_to_print[1], "o x o");
        assert_eq!(lines_to_print[2], "x o x");
    }

    #[test]
    fn should_get_neighbours_states() {
        let state = vec![
            "x o x",
            "o x o",
            "x o x"
        ];
        let universe = Universe::new_from_states(&state);

        let neighbour_states = universe.get_neighbours_states_of(1, 1);

        assert_eq!(neighbour_states, vec![
            CellState::ALIVE, CellState::DEAD, CellState::ALIVE,
            CellState::DEAD, CellState::DEAD,
            CellState::ALIVE, CellState::DEAD, CellState::ALIVE
        ]);
    }

    #[test]
    fn should_be_able_to_generate_a_linear_universe_of_two_cells_and_tick() {
        let universe = Universe::new(2, 1);
        println!("Before tick");
        print_universe(&universe);

        let new_universe = universe.tick();

        println!("After tick");
        print_universe(&new_universe);
        for line_to_print in new_universe.print_check() {
            assert_eq!(line_to_print, "(00)(1:E) (01)(1:W)");
        }
    }

    #[test]
    fn should_multiple_ticks() {
        let mut universe = Universe::new(10, 10);
        println!("Start");
        print_universe(&universe);

        let number_of_ticks = 10;
        for x in 0..=number_of_ticks {
            println!("Tick");
            let new_universe = universe.tick();
            print_universe(&new_universe);
            universe = new_universe
        }
    }

    mod game_rules {
        use crate::nopointer::universe::Universe;
        use crate::nopointer::universe::universe_tests::print_universe;

        // Any live cell with fewer than two live neighbours dies, as if caused by under-population.
        #[test]
        fn should_be_dead_when_have_one_neighbour_alive_at_next_tick() {
            let state = vec![
                "o x o",
                "o x o",
                "o o o"
            ];
            let universe = Universe::new_from_states(&state);

            let new_universe = universe.tick();

            print_universe(&new_universe);
            let lines_to_print = new_universe.print();
            assert_eq!(lines_to_print[1].split(" ").collect::<Vec<&str>>()[1], "o");
        }

        // Any live cell with two or three live neighbours lives on to the next generation.
        #[test]
        fn should_be_alive_when_have_two_or_three_neighbours_alive_at_next_tick() {
            let state = vec![
                "o x x",
                "o x x",
                "o o o"
            ];
            let universe = Universe::new_from_states(&state);

            let new_universe = universe.tick();

            print_universe(&new_universe);
            let lines_to_print = new_universe.print();
            assert_eq!(lines_to_print[1].split(" ").collect::<Vec<&str>>()[1], "x");
        }

        // Any live cell with more than three live neighbours dies, as if by overcrowding.
        #[test]
        fn should_be_dead_when_more_then_three_neighbours_alive_at_next_tick() {
            let state = vec![
                "o x x",
                "o x x",
                "o o x"
            ];
            let universe = Universe::new_from_states(&state);

            let new_universe = universe.tick();

            print_universe(&new_universe);
            let lines_to_print = new_universe.print();
            assert_eq!(lines_to_print[1].split(" ").collect::<Vec<&str>>()[1], "o");
        }

        // Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
        #[test]
        fn should_be_alive_when_three_live_neighbours_alive_at_next_tick() {
            let state = vec![
                "o x x",
                "o o x",
                "o o o"
            ];
            let universe = Universe::new_from_states(&state);

            let new_universe = universe.tick();

            print_universe(&new_universe);
            let lines_to_print = new_universe.print();
            assert_eq!(lines_to_print[1].split(" ").collect::<Vec<&str>>()[1], "x");
        }
    }

    fn print_universe(universe: &Universe) {
        for line_to_print in universe.print() {
            println!("{:?}", line_to_print);
        }
    }

    fn print_check_universe(universe: &Universe) {
        for line_to_print in universe.print_check() {
            println!("{:?}", line_to_print);
        }
    }
}