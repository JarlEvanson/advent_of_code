#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum PrincipalDirection {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl PrincipalDirection {
    pub const ALL: [Self; 8] = [
        Self::North,
        Self::NorthEast,
        Self::East,
        Self::SouthEast,
        Self::South,
        Self::SouthWest,
        Self::West,
        Self::NorthWest,
    ];

    pub const fn offset(&self) -> (isize, isize) {
        match self {
            Self::North => (0, -1),
            Self::NorthEast => (1, -1),
            Self::East => (1, 0),
            Self::SouthEast => (1, 1),
            Self::South => (0, 1),
            Self::SouthWest => (-1, 1),
            Self::West => (-1, 0),
            Self::NorthWest => (-1, -1),
        }
    }

    pub const fn opposite(&self) -> Self {
        self.clockwise().clockwise().clockwise().clockwise()
    }

    pub const fn clockwise(&self) -> Self {
        match self {
            Self::North => Self::NorthEast,
            Self::NorthEast => Self::East,
            Self::East => Self::SouthEast,
            Self::SouthEast => Self::South,
            Self::South => Self::SouthWest,
            Self::SouthWest => Self::West,
            Self::West => Self::NorthWest,
            Self::NorthWest => Self::North,
        }
    }

    pub const fn counterclockwise(&self) -> Self {
        self.clockwise()
            .clockwise()
            .clockwise()
            .clockwise()
            .clockwise()
            .clockwise()
            .clockwise()
    }
}

impl From<Direction> for PrincipalDirection {
    fn from(value: Direction) -> Self {
        match value {
            Direction::North => PrincipalDirection::North,
            Direction::East => PrincipalDirection::East,
            Direction::South => PrincipalDirection::South,
            Direction::West => PrincipalDirection::West,
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub const ALL: [Self; 4] = [Self::North, Self::East, Self::South, Self::West];

    pub const fn offset(&self) -> (isize, isize) {
        match self {
            Self::North => (0, -1),
            Self::East => (1, 0),
            Self::South => (0, 1),
            Self::West => (-1, 0),
        }
    }

    pub const fn opposite(&self) -> Self {
        self.clockwise().clockwise()
    }

    pub const fn clockwise(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    pub const fn counterclockwise(&self) -> Self {
        self.clockwise().clockwise().clockwise()
    }
}
