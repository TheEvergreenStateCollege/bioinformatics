use std::fmt;

/// A two dimensional array
#[derive(Debug, PartialEq, Eq)]
pub struct Matrix<T> {
    cells: Vec<Option<T>>,
    width: usize,
    height: usize,
}

impl<T> Matrix<T>
where
    T: Clone,
{
    /// Constructs a new `Matrix` with the given width and height.
    pub fn new(width: usize, height: usize) -> Self {
        Matrix {
            cells: vec![None; width * height],
            width,
            height,
        }
    }

    /// Get the value of the cell at position (x, y). Returns `None` if the position is out of
    /// bounds.
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        match self.cells.get((y * self.width) + x) {
            // Out of bounds
            None => None,
            // Nothing in the cell
            Some(None) => None,
            // Something in the cell
            Some(value) => value.as_ref(),
        }
    }

    /// Sets the cell value at the position (x, y). Panics if the position is out of bounds.
    pub fn set(&mut self, x: usize, y: usize, value: T) {
        if y >= self.height || x >= self.width {
            panic!("Out of bounds");
        } else {
            self.cells[(y * self.width) + x] = Some(value);
        }
    }
}

impl<T> fmt::Display for Matrix<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.cells.chunks(self.width) {
            for cell in row {
                match cell {
                    Some(value) => write!(f, "{} ", value)?,
                    None => write!(f, "- ")?,
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructor() {
        Matrix::<u8>::new(3, 3);
    }

    #[test]
    fn test_set_get() {
        let mut a: Matrix<u8> = Matrix::new(3, 3);
        a.set(0, 0, 0);

        assert_eq!(&0, a.get(0, 0).unwrap());
    }

    #[test]
    #[should_panic]
    fn test_set_out_of_bounds() {
        let mut m: Matrix<u8> = Matrix::new(3, 3);
        m.set(4, 4, 0);
    }

    #[test]
    fn test_get_out_of_bounds() {
        let m: Matrix<u8> = Matrix::new(3, 3);
        assert_eq!(None, m.get(4, 4));
    }

    #[test]
    fn test_display() {
        let mut m: Matrix<u8> = Matrix::new(3, 3);

        m.set(0, 0, 1);
        m.set(1, 0, 2);
        m.set(2, 0, 3);

        m.set(0, 1, 4);
        m.set(1, 1, 5);
        m.set(2, 1, 6);

        m.set(0, 2, 7);
        m.set(1, 2, 8);
        m.set(2, 2, 9);

        assert_eq!("1 2 3 \n4 5 6 \n7 8 9 \n", m.to_string());
    }
}
