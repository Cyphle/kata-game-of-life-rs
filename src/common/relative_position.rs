#[derive(Debug, PartialEq)]
pub enum RelativePosition {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
    Central,
}

impl RelativePosition {
    pub fn get_position_from(x: usize, y: usize, relative_x: usize, relative_y: usize) -> RelativePosition {
        let x_translation: i32 = x as i32 - relative_x as i32;
        let y_translation = y as i32 - relative_y as i32;

        match y_translation {
            -1 => {
                match x_translation {
                    -1 => RelativePosition::SouthEast,
                    0 => RelativePosition::South,
                    1 | _ => RelativePosition::SouthWest
                }
            },
            0 => {
                match x_translation {
                    -1 => RelativePosition::East,
                    0 => RelativePosition::Central,
                    1 | _ => RelativePosition::West,
                }
            },
            1 | _ => {
                match x_translation {
                    -1 => RelativePosition::NorthEast,
                    0 => RelativePosition::North,
                    1 | _ => RelativePosition::NorthWest
                }
            }
        }
    }

    pub fn print(&self) -> String {
        match self {
            RelativePosition::North => String::from("N"),
            RelativePosition::NorthEast => String::from("NE"),
            RelativePosition::East => String::from("E"),
            RelativePosition::SouthEast => String::from("SE"),
            RelativePosition::South => String::from("S"),
            RelativePosition::SouthWest => String::from("SW"),
            RelativePosition::West => String::from("W"),
            RelativePosition::NorthWest => String::from("NW"),
            RelativePosition::Central => String::from("C"),
        }
    }
}