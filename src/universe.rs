use crate::cell::Cell;

#[derive(Debug, PartialEq)]
struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>
}

impl Universe {
    fn print(&self) -> Vec<String> {
        let to_print: Vec<String> = self.cells.iter().map(|x| x.print()).collect();
        vec![to_print.join(" ")]
    }

    fn new(width: u32, height: u32) -> Universe {
        let cells = vec![Cell::new_alive()];
        Universe {
            width,
            height,
            cells
        }
    }
}

#[cfg(test)]
mod universe_tests {
    use crate::cell::Cell;
    use crate::universe::Universe;

    #[test]
    fn should_be_able_to_generate_a_monocellular_universe() {
        let universe = Universe::new(1, 1);

        assert_eq!(universe, Universe {
            width: 1,
            height: 1,
            cells: vec![Cell::new_alive()]
        })
    }

    #[test]
    fn should_print_universe() {
        let universe = Universe::new(1, 1);

        for line_to_print in universe.print() {
            println!("{:?}", line_to_print);
        }
    }
}