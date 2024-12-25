pub trait Direction: Sized {
    fn iter() -> impl Iterator<Item = Self>;
    fn as_dir(&self) -> (isize, isize);
    fn iter_dirs() -> impl Iterator<Item = (isize, isize)> {
        Self::iter().map(|d| d.as_dir())
    }
    fn rot_cw(&self) -> Self;
    fn rot_ac(&self) -> Self;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum D4 {
    N,
    E,
    S,
    W,
}

impl Direction for D4 {
    fn iter() -> impl Iterator<Item = Self> {
        [Self::N, Self::E, Self::S, Self::W].into_iter()
    }

    fn as_dir(&self) -> (isize, isize) {
        match self {
            Self::N => (-1, 0),
            Self::E => (0, 1),
            Self::S => (1, 0),
            Self::W => (0, -1),
        }
    }

    fn rot_cw(&self) -> Self {
        match self {
            Self::N => Self::E,
            Self::E => Self::S,
            Self::S => Self::W,
            Self::W => Self::N,
        }
    }

    fn rot_ac(&self) -> Self {
        match self {
            Self::N => Self::W,
            Self::E => Self::N,
            Self::S => Self::E,
            Self::W => Self::S,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum D8 {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Direction for D8 {
    fn iter() -> impl Iterator<Item = Self> {
        [
            Self::N,
            Self::NE,
            Self::E,
            Self::SE,
            Self::S,
            Self::SW,
            Self::W,
            Self::NW,
        ]
        .into_iter()
    }

    fn as_dir(&self) -> (isize, isize) {
        match self {
            Self::N => (-1, 0),
            Self::NE => (-1, 1),
            Self::E => (0, 1),
            Self::SE => (1, 1),
            Self::S => (1, 0),
            Self::SW => (1, -1),
            Self::W => (0, -1),
            Self::NW => (-1, -1),
        }
    }

    fn rot_cw(&self) -> Self {
        match self {
            Self::N => Self::NE,
            Self::NE => Self::E,
            Self::E => Self::SE,
            Self::SE => Self::S,
            Self::S => Self::SW,
            Self::SW => Self::W,
            Self::W => Self::NW,
            Self::NW => Self::N,
        }
    }

    fn rot_ac(&self) -> Self {
        match self {
            Self::N => Self::NW,
            Self::NE => Self::N,
            Self::E => Self::NE,
            Self::SE => Self::E,
            Self::S => Self::SE,
            Self::SW => Self::S,
            Self::W => Self::SW,
            Self::NW => Self::W,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Stepper<T, D> {
    min: (isize, isize),
    max: (isize, isize),
    unit: std::marker::PhantomData<T>,
    direction: std::marker::PhantomData<D>,
}

impl<D: Direction> Stepper<isize, D> {
    pub fn new(min: (isize, isize), max: (isize, isize)) -> Self {
        Self {
            min,
            max,
            unit: std::marker::PhantomData,
            direction: std::marker::PhantomData,
        }
    }
}

impl<D: Direction> Stepper<usize, D> {
    pub fn new_max(max: (usize, usize)) -> Self {
        Self {
            min: (0, 0),
            max: (max.0 as isize, max.1 as isize),
            unit: std::marker::PhantomData,
            direction: std::marker::PhantomData,
        }
    }
}

impl<T, D> Stepper<T, D> {
    fn pos_valid(&self, pos: (isize, isize)) -> bool {
        pos.0 >= self.min.0 && pos.1 >= self.min.0 && pos.0 < self.max.0 && pos.1 < self.max.1
    }
}

pub trait Step {
    type Type: Copy;
    type Direction: Direction;
    fn step(
        &self,
        pos: (Self::Type, Self::Type),
        direction: Self::Direction,
    ) -> Option<(Self::Type, Self::Type)>;

    fn neighbours(
        &self,
        pos: (Self::Type, Self::Type),
    ) -> impl Iterator<Item = (Self::Type, Self::Type)> {
        Self::Direction::iter().filter_map(move |d| self.step(pos, d))
    }
}

impl<D: Direction> Step for Stepper<usize, D> {
    type Type = usize;
    type Direction = D;

    fn step(
        &self,
        pos: (Self::Type, Self::Type),
        direction: Self::Direction,
    ) -> Option<(Self::Type, Self::Type)> {
        let dir = direction.as_dir();
        let p = (pos.0 as isize + dir.0, pos.1 as isize + dir.1);
        self.pos_valid(p).then_some((p.0 as usize, p.1 as usize))
    }
}

impl<D: Direction> Step for Stepper<isize, D> {
    type Type = isize;
    type Direction = D;

    fn step(
        &self,
        pos: (Self::Type, Self::Type),
        direction: Self::Direction,
    ) -> Option<(Self::Type, Self::Type)> {
        let dir = direction.as_dir();
        let p = (pos.0 + dir.0, pos.1 + dir.1);
        self.pos_valid(p).then_some(p)
    }
}
