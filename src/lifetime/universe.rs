// TODO
/*
La stratégie est de reconstruire un univers à chaque tick et de sortir l'ancien du scope pour qu'il soit détruit

 */

use crate::smartpointers::cell::Cell;

static UNIVERSE_START_INDEX: usize = 0;

#[derive(Debug, PartialEq)]
struct CellPosition<'a> {
    x: usize,
    y: usize,
    cell: &'a Cell,
}

#[derive(Debug, PartialEq)]
pub struct Universe<'a> {
    width: usize,
    height: usize,
    cells: Vec<Vec<CellPosition<'a>>>,
}

impl Universe {
    pub fn new(width: usize, height: usize) -> crate::smartpointers::universe::Universe {
        let states = Self::generate_base_states(width, height);
        crate::smartpointers::universe::Universe::new_with_defined_states(states)
    }

    pub fn print(&self) -> Vec<String> {
        self
            .cells
            .iter()
            .map(|x| x
                .iter()
                .map(|y| y.cell.print())
                // .map(|y| format!("{}({}{})({}:{})", y.cell.borrow().print(), y.y, y.x, y.cell.borrow().print_neighbours_count(), y.cell.borrow().print_neighbours_positions()))
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
                .map(|y| format!("({}{})({}:{})", y.y, y.x, y.cell.borrow().print_neighbours_count(), y.cell.borrow().print_neighbours_positions()))
                .collect::<Vec<String>>()
                .join(" ")
            )
            .collect()
    }
}

#[cfg(test)]
mod universe_tests {
    use crate::lifetime::universe::Universe;

    #[test]
    fn should_be_able_to_generate_a_monocellular_universe() {
        let universe = Universe::new(1, 1);

        for line_to_print in universe.print() {
            assert_eq!(line_to_print, "x");
            println!("{:?}", line_to_print);
        }
    }

    // #[test]
    // fn should_be_able_to_generate_a_linear_universe_of_two_cells() {
    //     let universe = Universe::new(2, 1);
    //
    //     print_universe(&universe);
    //     for line_to_print in universe.print_check() {
    //         assert_eq!(line_to_print, "(00)((1n):E) (01)((1n):W)");
    //     }
    // }
    //
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