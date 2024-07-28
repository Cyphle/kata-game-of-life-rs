use std::cell::RefCell;
use std::rc::Rc;
use crate::cell::Cell;
use rand::Rng;

static UNIVERSE_START_INDEX: u32 = 0;

#[derive(Debug, PartialEq)]
struct CellPosition {
    x: u32,
    y: u32,
    cell: Rc<RefCell<Cell>>,
}

#[derive(Debug, PartialEq)]
struct Universe {
    width: u32,
    height: u32,
    cells: Vec<CellPosition>,
}

impl Universe {
    fn tick(&self) {
        // TODO au tick il faut remplacer toutes les cellules
    }

    fn print(&self) -> Vec<String> {
        let to_print: Vec<String> = self.cells.iter().map(|x| x.cell.borrow().print()).collect();
        vec![to_print.join(" ")]
    }

    fn print_check(&self) -> Vec<String> {
        let to_print: Vec<String> = self.cells.iter().map(|_| "x".to_string()).collect();
        vec![to_print.join(" ")]
    }

    fn new(width: u32, height: u32) -> Universe {
        let mut cells = vec![];
        for x in UNIVERSE_START_INDEX..width {
            let state = rand::thread_rng().gen_range(0..2);
            cells.push(CellPosition {
                x,
                y: 1,
                cell: match state {
                    0 => Rc::new(RefCell::new(Cell::new_dead())),
                    _ => Rc::new(RefCell::new(Cell::new_alive())),
                }
            })
        }

        for x in &cells {
            // TODO Ajouter les voisins.
            /*
            Si x = 0, alors on est sur la première ligne
            Si x = universe.width - 1, alors on est sur la dernière ligne
            Si y = 0, alors on est sur la première colonne
            Si y = universe.height - 1, alors on est sur la derniere colonne
             */
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

        print_universe(&universe);
        for line_to_print in universe.print_check() {
            assert_eq!(line_to_print, "x x");
        }
    }

    #[test]
    fn should_be_able_to_generate_a_linear_universe_of_three_cells_with_random_state() {
        let universe = Universe::new(3, 1);

        print_universe(&universe);
        for line_to_print in universe.print_check() {
            assert_eq!(line_to_print, "x x x");
        }
    }

    #[test]
    fn should_be_able_to_generate_a_linear_universe_of_two_cells_and_tick() {
        let universe = Universe::new(2, 1);
        print_universe(&universe);

        universe.tick();

        print_universe(&universe);
        for line_to_print in universe.print_check() {
            assert_eq!(line_to_print, "x x");
        }
    }

    fn print_universe(universe: &Universe) {
        for line_to_print in universe.print() {
            println!("{:?}", line_to_print);
        }
    }
}