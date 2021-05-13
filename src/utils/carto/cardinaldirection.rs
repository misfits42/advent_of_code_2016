/// Represents one of the four cardinal directions.
#[derive(Copy, Clone)]
pub enum CardinalDirection {
    North,
    East,
    South,
    West
}

impl CardinalDirection {
    /// Determines new direction resulting from single 90-degree rotation to left (CCW).
    pub fn rotate_left(&self) -> CardinalDirection {
        match self {
            CardinalDirection::North => return CardinalDirection::West,
            CardinalDirection::East => return CardinalDirection::North,
            CardinalDirection::South => return CardinalDirection::East,
            CardinalDirection::West => return CardinalDirection::South,
        }
    }

    /// Determines new direction resulting from single 90-degree rotation to right (CW).
    pub fn rotate_right(&self) -> CardinalDirection {
        match self {
            CardinalDirection::North => return CardinalDirection::East,
            CardinalDirection::East => return CardinalDirection::South,
            CardinalDirection::South => return CardinalDirection::West,
            CardinalDirection::West => return CardinalDirection::North,
        }
    }
}
