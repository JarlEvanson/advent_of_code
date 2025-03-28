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

        Some(&self.data[column + row * self.width])
    }

    pub fn get_mut(&mut self, column: usize, row: usize) -> Option<&mut T> {
        if column >= self.width || row >= self.height {
            return None;
        }

        Some(&mut self.data[column + row * self.width])
    }

    pub fn get_signed(&self, column: isize, row: isize) -> Option<&T> {
        if column as usize >= self.width
            || row as usize >= self.height
            || column.is_negative()
            || row.is_negative()
        {
            return None;
        }

        Some(&self.data[column as usize + row as usize * self.width])
    }

    pub fn get_signed_mut(&mut self, column: isize, row: isize) -> Option<&mut T> {
        if column as usize >= self.width
            || row as usize >= self.height
            || column.is_negative()
            || row.is_negative()
        {
            return None;
        }

        Some(&mut self.data[column as usize + row as usize * self.width])
    }

    pub fn column(&self, column: usize) -> Option<Column<T>> {
        if column >= self.width() {
            return None;
        }

        Some(Column { grid: self, column })
    }

    pub fn columns(&self) -> ColumnsIter<T> {
        ColumnsIter {
            grid: self,
            column: 0,
        }
    }

    pub fn row(&self, row: usize) -> Option<Row<T>> {
        if row >= self.height() {
            return None;
        }

        Some(Row { grid: self, row })
    }

    pub fn rows(&self) -> RowsIter<T> {
        RowsIter { grid: self, row: 0 }
    }

    pub fn iter(&self) -> GridIter<T> {
        GridIter {
            grid: self,
            column: 0,
            row: 0,
        }
    }
}

pub struct ColumnsIter<'grid, T> {
    grid: &'grid Grid<T>,
    column: usize,
}

impl<'grid, T> Iterator for ColumnsIter<'grid, T> {
    type Item = Column<'grid, T>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.grid.column(self.column);
        if result.is_some() {
            self.column += 1;
        }

        result
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Column<'grid, T> {
    grid: &'grid Grid<T>,
    column: usize,
}

impl<'grid, T> Column<'grid, T> {
    pub fn get(&self, row: usize) -> Option<&'grid T> {
        if row >= self.grid.height() {
            return None;
        }

        Some(&self.grid.data[self.column + row * self.grid.width()])
    }
}

impl<'grid, T> IntoIterator for Column<'grid, T> {
    type IntoIter = ColumnIter<'grid, T>;
    type Item = &'grid T;

    fn into_iter(self) -> Self::IntoIter {
        ColumnIter {
            grid: self.grid,
            column: self.column,
            row: 0,
        }
    }
}

pub struct ColumnIter<'grid, T> {
    grid: &'grid Grid<T>,
    column: usize,
    row: usize,
}

impl<'grid, T> Iterator for ColumnIter<'grid, T> {
    type Item = &'grid T;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.grid.get(self.column, self.row);
        if result.is_some() {
            self.row += 1;
        }

        result
    }
}

pub struct RowsIter<'grid, T> {
    grid: &'grid Grid<T>,
    row: usize,
}

impl<'grid, T> Iterator for RowsIter<'grid, T> {
    type Item = Row<'grid, T>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.grid.row(self.row);
        if result.is_some() {
            self.row += 1;
        }

        result
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Row<'grid, T> {
    grid: &'grid Grid<T>,
    row: usize,
}

impl<'grid, T> Row<'grid, T> {
    pub fn get(&self, column: usize) -> Option<&'grid T> {
        if column >= self.grid.width() {
            return None;
        }

        Some(&self.grid.data[column + self.row * self.grid.width()])
    }
}

impl<'grid, T> IntoIterator for Row<'grid, T> {
    type IntoIter = RowIter<'grid, T>;
    type Item = &'grid T;

    fn into_iter(self) -> Self::IntoIter {
        RowIter {
            grid: self.grid,
            column: 0,
            row: self.row,
        }
    }
}

pub struct RowIter<'grid, T> {
    grid: &'grid Grid<T>,
    column: usize,
    row: usize,
}

impl<'grid, T> Iterator for RowIter<'grid, T> {
    type Item = &'grid T;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.grid.get(self.column, self.row);
        if result.is_some() {
            self.column += 1;
        }

        result
    }
}

pub struct GridIter<'grid, T> {
    grid: &'grid Grid<T>,
    column: usize,
    row: usize,
}

impl<'grid, T> Iterator for GridIter<'grid, T> {
    type Item = (usize, usize, &'grid T);

    fn next(&mut self) -> Option<Self::Item> {
        let result = self
            .grid
            .get(self.column, self.row)
            .map(|value| (self.column, self.row, value));
        if result.is_some() {
            self.column += 1;
            if self.column == self.grid.width() {
                self.column = 0;
                self.row += 1;
            }
        }

        result
    }
}
