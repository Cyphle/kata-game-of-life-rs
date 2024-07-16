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

    pub fn add_neighbour(&mut self, neighbour: &'a Cell) {
        self.neighbours.push(neighbour);
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
    fn should_not_be_able_to_add_two_neighbours_at_same_position<'a>() {
        // let mut cell = Cell::new_alive();
        // let mut neighbour_one = Cell::new_alive();
        // let mut neighbour_two = Cell::new_alive();
        //
        // cell.add_neighbour(&neighbour_one, RelativePosition::EAST);
        // cell.add_neighbour(&neighbour_one, RelativePosition::EAST);
        //
        // let east_neighbours = cell.neighbours.into_iter().filter(|(neighbour, position)| match position {
        //     RelativePosition::NORTH |
        //     RelativePosition::WEST |
        //     RelativePosition::SOUTH => {false}
        //     RelativePosition::EAST => {true}
        // })
        //     .collect::<(&Cell, RelativePosition)>();

        let mut test: Vec<&'a TestCell<'a>> = vec![];
        test.push(&TestCell::new_alive());
        test.push(&TestCell::new_alive());

        let res: Vec<bool> = test.iter().map(|c| c.is_alive()).collect();


        let toto = "";
    }

}