use crate::cell::Cell;
use rand::Rng;

static UNIVERSE_START_INDEX: u32 = 1;

#[derive(Debug, PartialEq)]
struct CellPosition {
    x: u32,
    y: u32,
    cell: Cell,
}

#[derive(Debug, PartialEq)]
struct Universe {
    width: u32,
    height: u32,
    cells: Vec<CellPosition>
}

impl Universe {
    fn print(&self) -> Vec<String> {
        let to_print: Vec<String> = self.cells.iter().map(|x| x.cell.print()).collect();
        vec![to_print.join(" ")]
    }

    fn print_check(&self) -> Vec<String> {
        let to_print: Vec<String> = self.cells.iter().map(|_| "x".to_string()).collect();
        vec![to_print.join(" ")]
    }

    fn new(width: u32, height: u32) -> Universe {
        let mut cells = vec![];
        for x in UNIVERSE_START_INDEX..=width {
            let state = rand::thread_rng().gen_range(0..2);
            cells.push(CellPosition {
                x,
                y: 1,
                cell: match state {
                    0 => Cell::new_dead(),
                    _ => Cell::new_alive(),
                }
            })
        }
        Universe {
            width,
            height,
            cells
        }
    }
}

#[cfg(test)]
mod universe_tests {
    use crate::universe::Universe;

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

        for line_to_print in universe.print() {
            assert_eq!(line_to_print, "x x");
            println!("{:?}", line_to_print);
        }
    }

    #[test]
    fn should_be_able_to_generate_a_linear_universe_of_three_cells_with_random_state() {
        let universe = Universe::new(3, 1);

        for line_to_print in universe.print_check() {
            assert_eq!(line_to_print, "x x x");
        }
        for line_to_print in universe.print() {
            println!("{:?}", line_to_print);
        }
    }
}