#[derive(Debug)]
pub enum TestCellState {
    ALIVE,
    DEAD,
}

#[derive(Debug)]
pub struct TestCell<'a> {
    state: TestCellState,
    neighbours: Vec<&'a TestCell<'a>>,
}

impl<'a> TestCell<'a> {
    pub fn is_alive(&self) -> bool {
        return match self.state {
            TestCellState::ALIVE => { true }
            TestCellState::DEAD => { false }
        };
    }

    pub fn add_neighbour(&mut self, neighbour: &'a TestCell) {
        self.neighbours.push(neighbour);
    }

    pub fn get_alives(&mut self) {
        let test: Vec<bool> = self.neighbours.iter().map(|cell| cell.is_alive()).collect();

        println!("test {:?}", test);

        let toto = "";
    }

    pub fn new_alive() -> TestCell<'a> {
        TestCell {
            state: TestCellState::ALIVE,
            neighbours: vec![],
        }
    }

    pub fn new_dead() -> TestCell<'a> {
        TestCell {
            state: TestCellState::DEAD,
            neighbours: vec![],
        }
    }
}

#[cfg(test)]
mod cell_tests {
    use crate::test_cell::TestCell;

    #[test]
    fn should_not_be_able_to_add_two_neighbours_at_same_position() {
        let mut cell = TestCell::new_alive();
        let a = TestCell::new_alive();
        let b = TestCell::new_dead();
        cell.add_neighbour(&a);
        cell.add_neighbour(&b);

        cell.get_alives();
    }

    #[test]
    fn should_iter_in_vec_of_struct() {
        let mut cells: Vec<TestCell> = vec![];
        cells.push(TestCell::new_alive());
        cells.push(TestCell::new_alive());

        println!("cells {:?}", cells);

        let res: Vec<TestCell> = cells
            .into_iter()
            .filter(|cell| cell.is_alive())
            .collect();

        println!("res {:?}", res);
    }

}