use std::cell::RefCell;
use std::rc::Rc;
use crate::cell::Cell;
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

impl Universe {
    fn tick(&self) {
        // TODO au tick il faut remplacer toutes les cellules
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
        // TODO Ajouter les voisins.
        /*
        Si x = 0, alors on est sur la première ligne
        Si x = universe.width - 1, alors on est sur la dernière ligne
        Si y = 0, alors on est sur la première colonne
        Si y = universe.height - 1, alors on est sur la derniere colonne

        Algo possible pour limiter la complexité et rester en O(n) ou O(2n)
        Pour n = 0 à x:
            Pour m = 0 à y:
                Crate Cell c à la position (x, y) avec un state aléatoire
                Ajouter les voisins à c,
                    Si x = 0, il n'y a pas de voisins à x - 1
                    Si x = width - 1, il n'y a pas de voisin à x + 1
                    Si y = 0, il n'y a pas de voisin à y - 1
                    Si y = height - 1, il n'y a pas de voisin à y + 1
                    En prenant en compte ces cas, il faut, pour x - 1 < p < x + 1 et y - 1 < q < y + 1, vérifier s'il y a quelqu'un et si oui ajouter
                    Si y a quelqu'un, il faut aussi ajouter c à ce voisin
         */

        let mut cells: Vec<Vec<CellPosition>> = vec![];
        for x in UNIVERSE_START_INDEX..width {
            let mut line = vec![];

            for y in UNIVERSE_START_INDEX..height {
                let state = rand::thread_rng().gen_range(0..2);
                let cell = match state {
                    0 => Rc::new(RefCell::new(Cell::new_dead())),
                    _ => Rc::new(RefCell::new(Cell::new_alive())),
                };
                //
                // TODO en fait au premier tour, x = 0 donc on peut pas faire x - 1
                let line_neighbours_start = if x > 0 { x - 1 } else { 0 };
                for p in line_neighbours_start..=x + 1 {
                    if p >= 0 && p < width {
                        //                 for q in y-1..=y+1 {
                        //                     if q >= 0 && q < height {
                        match cells.get(p) {
                            Some(current_line) => {
                                //                                 match current_column.get(q) {
                                //                                     Some(current_neighbour) => {
                                //                                         // current_neighbour.add_neighbour
                                println!("I am a neighbour");
                                //                                     }
                                //                                     _ => {}
                                //                                 }
                            }
                            _ => {}
                        }
                        //                     }
                        //                 }
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