use rand::Rng;

use crate::common::cell_state::CellState;
use crate::common::relative_position::RelativePosition;
use crate::lifetime_mut::cell::Cell;

static UNIVERSE_START_INDEX: usize = 0;

// TODO
/*
La stratégie est de reconstruire un univers à chaque tick et de sortir l'ancien du scope pour qu'il soit détruit

 */

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
    pub fn new(width: usize, height: usize) -> Universe {
        let states = Self::generate_base_states(width, height);
        Universe::new_with_defined_states(states)
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

    pub fn new_with_defined_states(states: Vec<Vec<CellState>>) -> Universe {
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

        let positions = (column_neighbours_start..=column_neighbours_end)
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
            .map(|inner| inner.join(" "))    // Join each inner vector with spaces
            .collect::<Vec<String>>()        // Collect it back into a Vec<String>
            .join("\n");

        positions
    }

    pub fn print(&self) -> Vec<String> {
        self
            .cells
            .iter()
            .map(|x| x
                .iter()
                .map(|y| y.cell.print())
                // .map(|y| format!("{}({}{})({}:{})", y.cell.print(), y.y, y.x, y.cell.print_neighbours_count(), y.cell.print_neighbours_positions()))
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
}

#[cfg(test)]
mod universe_tests {
    use crate::lifetime_mut::universe::Universe;

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

    // #[test]
    // fn should_be_able_to_generate_a_vertical_universe_of_two_cells() {
    //     let universe = Universe::new(1, 2);
    //
    //     print_universe(&universe);
    //     let lines_to_print = universe.print_check();
    //     assert_eq!(lines_to_print[0], "(00)((1n):S)");
    //     assert_eq!(lines_to_print[1], "(10)((1n):N)");
    // }
    //
    // #[test]
    // fn should_be_able_to_generate_a_square_universe_of_two_cells() {
    //     let universe = Universe::new(2, 2);
    //
    //     print_check_universe(&universe);
    //     let print_check = universe.print_check();
    //     assert_eq!(print_check[0], "(00)((3n):E,S,SE) (01)((3n):W,SW,S)");
    //     assert_eq!(print_check[1], "(10)((3n):N,NE,E) (11)((3n):NW,N,W)");
    // }
    //
    // #[test]
    // fn should_be_able_to_generate_a_linear_universe_of_three_cells_with_random_state() {
    //     let universe = Universe::new(3, 1);
    //
    //     print_universe(&universe);
    //     for line_to_print in universe.print_check() {
    //         assert_eq!(line_to_print, "(00)((1n):E) (01)((2n):W,E) (02)((1n):W)");
    //     }
    // }
    //
    // #[test]
    // fn should_be_able_to_generate_a_vertical_universe_of_three_cells_with_random_state() {
    //     let universe = Universe::new(3, 1);
    //
    //     print_universe(&universe);
    //     let print_check = universe.print_check();
    //     assert_eq!(print_check[0], "(00)((1n):E) (01)((2n):W,E) (02)((1n):W)");
    // }
    //
    // #[test]
    // fn should_be_able_to_generate_a_square_universe_of_three_cells() {
    //     let universe = Universe::new(3, 3);
    //
    //     print_check_universe(&universe);
    //     let lines_to_print = universe.print_check();
    //     assert_eq!(lines_to_print[0], "(00)((3n):E,S,SE) (01)((5n):W,E,SW,S,SE) (02)((3n):W,SW,S)");
    //     assert_eq!(lines_to_print[1], "(10)((5n):N,NE,E,S,SE) (11)((8n):NW,N,NE,W,E,SW,S,SE) (12)((5n):NW,N,W,SW,S)");
    //     assert_eq!(lines_to_print[2], "(20)((3n):N,NE,E) (21)((5n):NW,N,NE,W,E) (22)((3n):NW,N,W)");
    // }
    //
    // #[test]
    // fn should_be_able_to_generate_a_linear_universe_of_two_cells_and_tick() {
    //     let universe = Universe::new(2, 1);
    //     println!("Before tick");
    //     print_universe(&universe);
    //
    //     universe.tick();
    //
    //     println!("After tick");
    //     print_universe(&universe);
    //     for line_to_print in universe.print_check() {
    //         assert_eq!(line_to_print, "(00)((1n):E) (01)((1n):W)");
    //     }
    // }
    //
    // #[test]
    // fn should_multiple_ticks() {
    //     let universe = Universe::new(10, 10);
    //     println!("Start");
    //     print_universe(&universe);
    //
    //     let number_of_ticks = 10;
    //     for x in 0..=number_of_ticks {
    //         println!("Tick");
    //         universe.tick();
    //         print_universe(&universe);
    //     }
    // }

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