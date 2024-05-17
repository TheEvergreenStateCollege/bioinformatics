use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Array2D<T> {
    array: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Array2D<T>
where
    T: Default + Clone,
{
    pub fn new(width: usize, height: usize) -> Self {
        Array2D {
            // TODO: The array should store Option
            array: vec![T::default(); width * height],
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
                write!(f, "{} ", col)?;
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
        Array2D::<i32>::new(3, 3);
    }

    #[test]
    fn test_set_get() {
        let mut a: Array2D<i32> = Array2D::new(3, 3);
        a.set(0, 0, 0);

        assert_eq!(&0, a.get(0, 0).unwrap());
    }

    #[test]
    #[should_panic]
    fn test_set_out_of_bounds() {
        let mut a: Array2D<i32> = Array2D::new(3, 3);
        a.set(4, 4, 0);
    }

    #[test]
    fn test_get_out_of_bounds() {
        let a: Array2D<i32> = Array2D::new(3, 3);
        assert_eq!(None, a.get(4, 4));
    }

    #[test]
    fn test_display() {
        let mut a: Array2D<i32> = Array2D::new(3, 3);
        a.set(0, 0, 1);
        a.set(0, 1, 2);
        a.set(0, 2, 3);

        a.set(1, 0, 4);
        a.set(1, 1, 5);
        a.set(1, 2, 6);

        a.set(2, 0, 7);
        a.set(2, 1, 8);
        a.set(2, 2, 9);

        assert_eq!("1 2 3 \n4 5 6 \n7 8 9 \n", a.to_string());
    }
}
