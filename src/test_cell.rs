pub enum TestCellState {
    ALIVE,
    DEAD,
}

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

    pub fn get_alives(&self) -> Vec<&TestCell> {
        return self.neighbours
            .iter()
            .filter(|cell| cell.is_alive())
            .collect();
    }

    pub fn new_alive() -> TestCell<'a> {
        TestCell {
            state: TestCellState::ALIVE,
            neighbours: vec![],
        }
    }
}

#[cfg(test)]
mod cell_tests {
    use crate::test_cell::TestCell;

    #[test]
    fn should_not_be_able_to_add_two_neighbours_at_same_position() {
        let cell = TestCell::new_alive();
        cell.ad(&TestCell::new_alive());
        cell.push(&TestCell::new_alive());

        let res = cell.get_alives();

        println!("{:?}", res);
        let toto = "";
    }

}