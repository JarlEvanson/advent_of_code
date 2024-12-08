#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Grid<T> {
    data: Box<[T]>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    pub fn new(data: Box<[T]>, width: usize, height: usize) -> Self {
        Self {
            data,
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, column: usize, row: usize) -> Option<&T> {
        if column >= self.width || row >= self.height {
            return None;
        }

        Some(&self.data[column + row * self.height])
    }

    pub fn get_mut(&mut self, column: usize, row: usize) -> Option<&mut T> {
        if column >= self.width || row >= self.height {
            return None;
        }

        Some(&mut self.data[column + row * self.height])
    }

    pub fn get_signed(&self, column: isize, row: isize) -> Option<&T> {
        if column as usize >= self.width
            || row as usize >= self.height
            || column.is_negative()
            || row.is_negative()
        {
            return None;
        }

        Some(&self.data[column as usize + row as usize * self.height])
    }

    pub fn get_signed_mut(&mut self, column: isize, row: isize) -> Option<&mut T> {
        if column as usize >= self.width
            || row as usize >= self.height
            || column.is_negative()
            || row.is_negative()
        {
            return None;
        }

        Some(&mut self.data[column as usize + row as usize * self.height])
    }
}
