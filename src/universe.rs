use std::cell::RefCell;
use std::rc::Rc;
use crate::cell::{Cell, RelativePosition};
use rand::Rng;

static UNIVERSE_START_INDEX: usize = 0;

#[derive(Debug, PartialEq)]
struct CellPosition {
    x: usize,
    y: usize,
    cell: Rc<RefCell<Cell>>,
}

#[derive(Debug, PartialEq)]
struct Universe {
    width: usize,
    height: usize,
    cells: Vec<Vec<CellPosition>>,
}

// TODO il va manquer un moyen de fournir des cellules de base
impl Universe {
    // TODO faut passer par des clones sinon le nouvel état de chaque cellule impact ses voisines alors que toutes les cellules doivent tick en même temps à partir de l'état n
    fn tick(&self) {
        for c_x in &self.cells {
            for c_y in c_x {
                c_y.cell.borrow_mut().pretick();
            }
        }
        for c_x in &self.cells {
            for c_y in c_x {
                c_y.cell.borrow_mut().tick();
            }
        }
    }

    fn print(&self) -> Vec<String> {
        let to_print: Vec<String> = self
            .cells
            .iter()
            .map(|x| x.iter().map(|y|
                y.cell.borrow().print()).collect::<Vec<String>>().join(" ")
            )
            .collect();
        to_print
    }

    fn print_check(&self) -> Vec<String> {
        let to_print: Vec<String> = self.cells.iter().map(|_| "x".to_string()).collect();
        vec![to_print.join(" ")]
    }

    fn new(width: usize, height: usize) -> Universe {
        let mut cells: Vec<Vec<CellPosition>> = vec![];
        // TODO le width et height sont inverses
        for x in UNIVERSE_START_INDEX..width {
            let mut line = vec![];

            for y in UNIVERSE_START_INDEX..height {
                let state = rand::thread_rng().gen_range(0..2);
                let cell = match state {
                    0 => Rc::new(RefCell::new(Cell::new_dead())),
                    _ => Rc::new(RefCell::new(Cell::new_alive())),
                };

                let line_neighbours_start = if x > 0 { x - 1 } else { 0 };
                for p in line_neighbours_start..=x + 1 {
                    if p < width {
                        let column_neighbours_start = if y > 0 { y - 1 } else { 0 };
                        for q in column_neighbours_start..=y + 1 {
                            if q < height {
                                match cells.get(p) {
                                    Some(current_line) => {
                                        match current_line.get(q) {
                                            Some(current_neighbour) => {
                                                // TODO en fonction de x, y et p, q déterminer la position relative
                                                cell.borrow_mut().add_neighbour(Rc::clone(&current_neighbour.cell), RelativePosition::North);
                                                // TODO en fonction de x, y et p, q déterminer la position relative
                                                current_neighbour.cell.borrow_mut().add_neighbour(Rc::clone(&&cell), RelativePosition::South);
                                            }
                                            _ => {}
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }

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
    fn should_be_able_to_generate_a_square_universe_of_two_cells() {
        let universe = Universe::new(2, 2);

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
        println!("Before tick");
        print_universe(&universe);

        universe.tick();

        println!("After tick");
        print_universe(&universe);
        for line_to_print in universe.print_check() {
            assert_eq!(line_to_print, "x x");
        }
    }

    #[test]
    fn should_multiple_ticks() {
        let universe = Universe::new(3, 3);
        println!("Start");
        print_universe(&universe);

        let number_of_ticks = 10;
        for x in 0..=number_of_ticks {
            println!("Tick");
            print_universe(&universe);
        }
    }

    fn print_universe(universe: &Universe) {
        for line_to_print in universe.print() {
            println!("{:?}", line_to_print);
        }
    }
}