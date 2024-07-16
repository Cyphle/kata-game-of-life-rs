#[derive(Debug)]
pub enum TestCellState {
    ALIVE,
    DEAD,
}

#[derive(Debug)]
pub struct TestCell {
    state: TestCellState,
    neighbours: Vec<TestCell>,
}

impl TestCell {
    pub fn is_alive(&self) -> bool {
        return match self.state {
            TestCellState::ALIVE => { true }
            TestCellState::DEAD => { false }
        };
    }

    pub fn add_neighbour(&mut self, neighbour: TestCell) {
        self.neighbours.push(neighbour);
    }

    // pub fn get_alives(&self) -> Vec<TestCell> {
    //     return self.neighbours
    //         .iter()
    //         .filter(|cell| cell.is_alive())
    //         .collect::<Vec<TestCell>>();
    // }

    pub fn new_alive() -> TestCell {
        TestCell {
            state: TestCellState::ALIVE,
            neighbours: vec![],
        }
    }
}

#[cfg(test)]
mod cell_tests {
    use crate::test_cell::TestCell;

    // #[test]
    // fn should_not_be_able_to_add_two_neighbours_at_same_position() {
    //     let mut cell = TestCell::new_alive();
    //     cell.add_neighbour(TestCell::new_alive());
    //     cell.add_neighbour(TestCell::new_alive());
    //
    //     let res = cell.get_alives();
    //
    //     println!("{:?}", res);
    //     let toto = "";
    // }

    #[test]
    fn should_iter_in_vec_of_struct() {
        let mut cells: Vec<TestCell> = vec![];
        cells.push(TestCell::new_alive());
        cells.push(TestCell::new_alive());

        let res: Vec<TestCell> = cells
            .into_iter()
            .filter(|cell| cell.is_alive())
            .collect();

        let toto = "";
    }

}