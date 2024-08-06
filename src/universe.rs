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
        self
            .cells
            .iter()
            .map(|x| x
                .iter()
                .map(|y| y.cell.borrow().print())
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
                .map(|y| format!("x{}", y.cell.borrow().print_neighbours_count()))
                .collect::<Vec<String>>()
                .join(" ")
            )
            .collect()
    }

    // TODO algo pour faire le parcours qu'une fois
    fn new(width: usize, height: usize) -> Universe {
        let mut cells: Vec<Vec<CellPosition>> = vec![];

        for y in UNIVERSE_START_INDEX..height {
            let mut line: Vec<CellPosition> = vec![];
            for x in UNIVERSE_START_INDEX..width {
                let cell = Rc::new(RefCell::new(Cell::new_random_state()));

                // TODO Peut être que les voisins sont inverses aussi...
                // TODO Le nombre de voisin n'est pas bon
                // TODO on ne regarde pas la ligne courante en train de se remplir en fait
                // Si p == x, il faut regarder la ligne courante
                Self::add_neighbours(width, height, &mut cells, y, &mut line, x, &cell);

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

    // TODO Peut être que les voisins sont inverses aussi...
    // TODO Le nombre de voisin n'est pas bon
    // TODO on ne regarde pas la ligne courante en train de se remplir en fait
    // Si p == x, il faut regarder la ligne courante
    fn add_neighbours(width: usize, height: usize, cells: &mut Vec<Vec<CellPosition>>, y: usize, line: &mut Vec<CellPosition>, x: usize, cell: &Rc<RefCell<Cell>>) {
        let column_neighbours_start = if y > 0 { y - 1 } else { 0 };
        for q in column_neighbours_start..=y + 1 {
            if q < height {
                if q == y { // Si on est sur la ligne en train d'être remplie
                    let line_neighbours_start = if x > 0 { x - 1 } else { 0 };
                    for p in line_neighbours_start..=x + 1 {
                        if p < width {
                            match line.get(p) {
                                Some(current_neighbour) => {
                                    // TODO en fonction de x, y et p, q déterminer la position relative
                                    cell.borrow_mut().add_neighbour(Rc::clone(&current_neighbour.cell), RelativePosition::North);
                                    // TODO en fonction de x, y et p, q déterminer la position relative
                                    current_neighbour.cell.borrow_mut().add_neighbour(Rc::clone(&&&cell), RelativePosition::South);
                                }
                                _ => {}
                            }
                        }
                    }
                } else {
                    match cells.get(q) {
                        Some(current_line) => {
                            let line_neighbours_start = if x > 0 { x - 1 } else { 0 };
                            for p in line_neighbours_start..=x + 1 {
                                if p < width {
                                    match current_line.get(p) {
                                        Some(current_neighbour) => {
                                            // TODO en fonction de x, y et p, q déterminer la position relative
                                            cell.borrow_mut().add_neighbour(Rc::clone(&current_neighbour.cell), RelativePosition::North);
                                            // TODO en fonction de x, y et p, q déterminer la position relative
                                            current_neighbour.cell.borrow_mut().add_neighbour(Rc::clone(&&&cell), RelativePosition::South);
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
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
    fn should_be_able_to_generate_a_vertical_universe_of_two_cells() {
        let universe = Universe::new(1, 2);

        print_universe(&universe);
        for line_to_print in universe.print_check() {
            assert_eq!(line_to_print, "x(1n)");
        }
    }

    #[test]
    fn should_be_able_to_generate_a_square_universe_of_two_cells() {
        let universe = Universe::new(2, 2);

        print_universe(&universe);
        let print_check = universe.print_check();
        assert_eq!(print_check[0], "x(3n) x(3n)");
        assert_eq!(print_check[1], "x(3n) x(3n)");
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
    fn should_be_able_to_generate_a_vertical_universe_of_three_cells_with_random_state() {
        let universe = Universe::new(3, 1);

        print_universe(&universe);
        let print_check = universe.print_check();
        assert_eq!(print_check[0], "x(1n)");
        assert_eq!(print_check[1], "x(2n)");
        assert_eq!(print_check[2], "x(1n)");
    }

    #[test]
    fn should_be_able_to_generate_a_square_universe_of_three_cells() {
        let universe = Universe::new(3, 3);

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

        let number_of_ticks = 1;
        for x in 0..=number_of_ticks {
            println!("Tick");
            universe.tick();
            print_universe(&universe);
        }
    }

    fn print_universe(universe: &Universe) {
        for line_to_print in universe.print() {
            println!("{:?}", line_to_print);
        }
    }
}