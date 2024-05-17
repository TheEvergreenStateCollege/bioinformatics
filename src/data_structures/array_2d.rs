use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Array2D<T> {
    array: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Array2D<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Array2D {
            array: Vec::with_capacity(width * height),
            width,
            height,
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        self.array.get((row * self.width) + col)
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) {
        if row >= self.height || col >= self.width {
            panic!("Out of bounds");
        } else {
            self.array[(row * self.width) + col] = value;
        }
    }
}

impl<T> fmt::Display for Array2D<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.array.chunks(self.width) {
            for col in row {
                write!(f, "{}", col)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructor() {
        let a: Array2D<i32> = Array2D::new(3, 3);
    }
}
