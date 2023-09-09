use std::fmt;
use std::ops::{Index, IndexMut};

/// A module containing Matrix and Matrix Functions.
pub struct Matrix<const ROWS: usize, const COLS: usize>(Vec<Vec<f64>>);

impl<const ROWS: usize, const COLS: usize> Matrix<ROWS, COLS> {
    /// Creates a new matrix from a 2D array of elements.
    ///
    /// # Parameters
    ///
    /// - `elements`: A 2D array representing the elements of the matrix. The outer array represents rows, and the inner arrays represent columns.
    ///
    /// # Returns
    ///
    /// A new matrix containing the elements provided in the 2D array.
    ///
    /// # Example
    ///
    /// ```
    /// use mathematica::Matrix;
    ///
    /// fn main() {
    ///     let matrix = Matrix::new([
    ///         [1_f64, 2_f64],
    ///         [3_f64, 4_f64]
    ///     ]);
    ///     
    ///     println!("Matrix:\n {:?}", matrix);
    /// }
    ///
    /// /*
    ///     Outputs the following:
    ///     ----------------------
    ///     Matrix:
    ///     [1, 2]
    ///     [3, 4]
    /// */
    /// ```
    /// <hr/>
    pub fn new(elements: [[f64; COLS]; ROWS]) -> Self {
        Self(elements.iter().map(|row| row.to_vec()).collect())
    }

    /// Updates the value at a specific row and column in the matrix.
    ///
    /// # Parameters
    ///
    /// - `row`: The row index (1-based) where the value should be updated.
    /// - `column`: The column index (1-based) where the value should be updated.
    /// - `new_value`: The new value to set at the specified position.
    ///
    /// If either the row or column index is out of bounds, this function does nothing.
    ///
    /// # Example:
    ///
    /// ```
    /// use mathematica::Matrix;
    ///
    /// fn main() {
    ///     let mut matrix = Matrix::new([
    ///         [1_f64, 2_f64, 3_f64],
    ///         [4_f64, 5_f64, 6_f64],
    ///         [7_f64, 8_f64, 9_f64]
    ///     ]);
    ///     
    ///     println!("Matrix:\n {:?}", matrix);    
    ///
    ///     matrix.update(2, 2, 10_f64);
    ///
    ///     println!("Updated Matrix:\n {:?}", matrix);
    /// }
    ///
    /// /*
    ///     Outputs the Following:
    ///     ----------------------
    ///     Matrix:
    ///     [1, 2, 3]
    ///     [4, 5, 6]
    ///     [7, 8, 9]
    ///
    ///     Updated Matrix:
    ///     [1, 2, 3]
    ///     [4, 10, 6]
    ///     [7, 8, 9]
    /// */
    /// ```
    /// <hr/>
    pub fn update(&mut self, row: usize, column: usize, new_value: f64) {
        if let Some(row_vec) = self.0.get_mut(row - 1) {
            if let Some(element) = row_vec.get_mut(column - 1) {
                *element = new_value;
            }
        }
    }

    /// Calculates the determinant of a square matrix using the Leibniz formula.
    ///
    /// # Returns
    ///
    /// The determinant of the matrix as an `f64` value.
    ///
    /// # Example:
    ///
    /// ```
    /// use mathematica::Matrix;
    ///
    /// fn main() {
    ///     let matrix = Matrix::new([
    ///         [78_f64, 94_f64, 25_f64, 1_f64],
    ///         [795_f64, 64_f64, 25_f64, 12_f64],
    ///         [37_f64, 52_f64, 81_f64, 64_f64],
    ///         [0_f64, 15_f64, 6_f64, 4_f64]
    ///     ]);
    ///     
    ///     let determinate = matrix.determinant();
    ///
    ///     println!("Matrix:\n {:?}\nDeterminate of the Matrix: {:?}", matrix, determinate);
    /// }
    ///
    /// /*
    ///     Outputs the Following:
    ///     ----------------------
    ///     Matrix:
    ///     [78, 94, 25, 1]
    ///     [795, 64, 25, 12]
    ///     [37, 52, 81, 64]
    ///     [0, 15, 6, 4]
    ///     Determinate of the Matrix:
    ///     -9395226.0
    /// */
    /// ```
    /// <hr/>
    pub fn determinant(&self) -> f64 {
        let n = self.0.len();
        if n != self.0[0].len() {
            panic!("Matrix is not square!");
        }

        if n == 1 {
            return self[0][0];
        }

        let mut sum = 0.0;
        let mut sign = 1.0;

        for i in 0..n {
            let submatrix: Vec<Vec<f64>> = self
                .0
                .iter()
                .enumerate()
                .filter_map(|(index, row)| {
                    if index != i {
                        Some(row[1..].to_vec())
                    } else {
                        None
                    }
                })
                .collect();

            sum += sign * self[i][0] * Matrix::<ROWS, COLS>::determinant(&Matrix(submatrix));
            sign *= -1.0;
        }

        sum
    }

    /// Returns the transpose of the matrix, where rows become columns and columns become rows.
    ///
    /// # Returns
    ///
    /// The transposed matrix.
    ///
    /// # Example:
    ///
    /// ```
    /// use mathematica::Matrix;
    ///
    /// fn main() {
    ///     let matrix = Matrix::new([
    ///         [78_f64, 94_f64, 25_f64, 1_f64],
    ///         [795_f64, 64_f64, 25_f64, 12_f64],
    ///         [37_f64, 52_f64, 81_f64, 64_f64],
    ///         [0_f64, 15_f64, 6_f64, 4_f64]
    ///     ]);
    ///
    ///     let transposed_matrix = matrix.transpose();
    ///
    ///     println!("Matrix:\n{:?}\nTransposed Matrix:\n{:?}", matrix, transposed_matrix)
    /// }
    ///
    /// /*
    ///     Outputs the Following:
    ///     ----------------------
    ///     Matrix:    
    ///     [78, 94, 25, 1]  
    ///     [795, 64, 25, 12]
    ///     [37, 52, 81, 64]
    ///     [0, 15, 6, 4]
    ///     Transposed Matrix:
    ///     [78, 795, 37, 0]
    ///     [94, 64, 52, 15]
    ///     [25, 25, 81, 6]
    ///     [1, 12, 64, 4]
    /// */
    /// ```
    /// <hr/>
    pub fn transpose(&self) -> Matrix<COLS, ROWS> {
        let rows = self.0.len();
        let columns = self.0[0].len();
        let mut transposed_matrix = Matrix::<COLS, ROWS>::new([[0.0; ROWS]; COLS]);

        for i in 0..rows {
            for j in 0..columns {
                transposed_matrix[j][i] = self[i][j];
            }
        }

        transposed_matrix
    }

    /// Returns the dimensions of the matrix as a vector where the first element is the number of rows and the second element is the number of columns.
    ///
    /// # Returns
    ///
    /// A vector containing the number of rows and columns, respectively.
    ///
    /// # Example:
    ///
    /// ```
    /// use mathematica::Matrix;
    ///
    /// fn main() {
    ///     let matrix = Matrix::new([
    ///         [1_f64, 2_f64],
    ///         [3_f64, 4_f64]
    ///     ]);
    ///
    ///     let dimensions = matrix.dimensions();
    ///
    ///     println!("Matrix:\n{:?}\nMatrix Dimensions:\n{:?}", matrix, dimensions);
    /// }
    ///
    /// /*
    ///     Outputs the Following:
    ///     ----------------------
    ///     Matrix:
    ///     [1, 2]
    ///     [3, 4]
    ///     Matrix Dimensions:
    ///     [2, 2]
    /// */
    /// ```
    /// <hr/>
    pub fn dimensions(&self) -> Vec<usize> {
        vec![ROWS, COLS]
    }

    /// Returns the identity matrix of the same size as the current matrix.
    ///
    /// # Returns
    ///
    /// An identity matrix of the same size as the current matrix.
    ///
    /// # Example
    ///
    /// ```
    /// use mathematica::Matrix;
    ///
    /// fn main() {
    ///     let identity_5x5 = Matrix::<5, 5>::identity();
    ///
    ///     println!("5x5 Identity Matrix:\n{:?}", identity_5x5);
    /// }
    ///
    /// /*
    ///     Outputs the Following:
    ///     ----------------------
    ///     5x5 Identity Matrix:
    ///     [1, 0, 0, 0, 0]
    ///     [0, 1, 0, 0, 0]
    ///     [0, 0, 1, 0, 0]
    ///     [0, 0, 0, 1, 0]
    ///     [0, 0, 0, 0, 1]
    /// */
    /// ```
    /// <hr/>
    pub fn identity() -> Matrix<ROWS, COLS> {
        let mut identity_matrix = Matrix::new([[0.0; COLS]; ROWS]);
        for i in 0..ROWS {
            if i < COLS {
                identity_matrix[i][i] = 1.0;
            }
        }
        identity_matrix
    }

    /// Calculates the cumulative sum of a matrix's rows.
    ///
    /// # Returns
    ///
    /// A new matrix where each row contains the cumulative sum of the corresponding row in the original matrix.
    ///
    /// # Example:
    ///
    /// ```
    /// use mathematica::Matrix;
    ///
    /// fn main() {
    ///     let matrix = Matrix::new([
    ///         [1_f64, 2_f64, 3_f64],
    ///         [4_f64, 5_f64, 6_f64],
    ///         [7_f64, 8_f64, 9_f64]
    ///     ]);
    ///
    ///     let cumsum_rows = matrix.cumsumr();
    ///
    ///     println!("Matrix:\n{:?}\nCumSum of Rows:\n{:?}", matrix, cumsum_rows);
    /// }
    ///
    /// /*
    ///     Outputs the Following:
    ///     ----------------------
    ///     Matrix:
    ///     [1, 2, 3]
    ///     [4, 5, 6]
    ///     [7, 8, 9]
    ///     CumSum of Rows:
    ///     [1, 3, 6]
    ///     [4, 9, 15]
    ///     [7, 15, 24]
    /// */
    /// ```
    /// <hr/>
    pub fn cumsumr(&self) -> Matrix<ROWS, COLS> {
        let mut cumsum_rows_matrix = self.0.clone();
        for row in &mut cumsum_rows_matrix {
            for j in 1..row.len() {
                row[j] += row[j - 1];
            }
        }
        Matrix(cumsum_rows_matrix)
    }

    /// Calculates the cumulative sum of a matrix's columns.
    ///
    /// # Returns
    ///
    /// A new matrix where each column contains the cumulative sum of the corresponding column in the original matrix.
    ///
    /// # Example:
    ///
    /// ```
    /// use mathematica::Matrix;
    ///
    /// fn main() {
    ///    let matrix = Matrix::new([
    ///         [1_f64, 2_f64, 3_f64],
    ///         [4_f64, 5_f64, 6_f64],
    ///         [7_f64, 8_f64, 9_f64]
    ///     ]);
    ///
    ///     let cumsum_columns = matrix.cumsumc();
    ///
    ///     println!("Matrix:\n{:?}\nCumSum of Columns:\n{:?}", matrix, cumsum_columns);  
    /// }
    ///
    /// /*
    ///     Outputs the Following:
    ///     ----------------------
    ///     Matrix:
    ///     [1, 2, 3]
    ///     [4, 5, 6]
    ///     [7, 8, 9]
    ///     CumSum of Columns:
    ///     [1, 2, 3]
    ///     [5, 7, 9]
    ///     [12, 15, 18]
    /// */
    /// ```
    /// <hr/>
    pub fn cumsumc(&self) -> Matrix<ROWS, COLS> {
        let transposed = self.transpose();
        let mut cumsum_columns_matrix = transposed.0.clone();
        for row in &mut cumsum_columns_matrix {
            for j in 1..row.len() {
                row[j] += row[j - 1];
            }
        }
        let transposed_cumsum = Matrix(cumsum_columns_matrix);
        transposed_cumsum.transpose()
    }

    /// Performs various arithmetic operations element-wise on two matrices.
    ///
    /// # Parameters
    ///
    /// - `self`: The first matrix.
    /// - `other`: The second matrix.
    /// - `op`: A closure that takes two `f64` values and returns an `f64`, representing the arithmetic operation to be performed.
    ///
    /// # Returns
    ///
    /// A new matrix containing the result of the element-wise arithmetic operation.
    ///
    /// # Example
    ///
    /// ```
    /// use mathematica::Matrix;
    ///
    /// fn main() {
    ///
    ///     let matrix1 = Matrix::new([
    ///         [1_f64, 2_f64],
    ///         [3_f64, 4_f64]
    ///     ]);
    ///
    ///     let matrix2 = Matrix::new([
    ///         [5_f64, 6_f64],
    ///         [7_f64, 8_f64]
    ///     ]);
    ///
    ///     let result_exponentiation = Matrix::arithmetic(&matrix1, &matrix2, |a, b| a.powf(b));
    ///     let result_addition = Matrix::arithmetic(&matrix1, &matrix2, |a, b| a + b);
    ///     let result_subtraction = Matrix::arithmetic(&matrix1, &matrix2, |a, b| a - b);
    ///     let result_multiplication = Matrix::arithmetic(&matrix1, &matrix2, |a, b| a * b);
    ///     let result_division = Matrix::arithmetic(&matrix1, &matrix2, |a, b| a / b);
    ///
    ///     println!("
    ///         Matrix 1:\n{:?}\n
    ///         Matrix 2:\n{:?}\n
    ///         Addition:\n{:?}\n
    ///         Subtraction:\n{:?}\n
    ///         Multiplication:\n{:?}\n
    ///         Division:\n{:?}\n
    ///         Power:\n{:?}", matrix1, matrix2, result_addition, result_subtraction, result_multiplication, result_division, result_exponentiation);
    /// }
    ///
    /// /*
    ///     Outputs the Following:
    ///     ----------------------
    ///     Matrix 1:
    ///     [1, 2]
    ///     [3, 4]
    ///     Matrix 2:
    ///     [5, 6]
    ///     [7, 8]
    ///     Addition:
    ///     [6, 8]
    ///     [10, 12]
    ///     Subtraction:
    ///     [-4, -4]
    ///     [-4, -4]
    ///     Multiplication:
    ///     [5, 12]
    ///     [21, 32]
    ///     Division:
    ///     [0.2, 0.333333333]
    ///     [0.428571428, 0.5]
    ///     Power:
    ///     [1, 64]
    ///     [2187, 65536]
    /// */
    ///
    /// ```
    /// <hr/>
    pub fn arithmetic<F>(&self, other: &Matrix<ROWS, COLS>, op: F) -> Matrix<ROWS, COLS>
    where
        F: Fn(f64, f64) -> f64,
    {
        let mut result_matrix = Matrix::new([[0.0; COLS]; ROWS]);

        for i in 0..ROWS {
            for j in 0..COLS {
                result_matrix[i][j] = op(self[i][j], other[i][j]);
            }
        }

        result_matrix
    }

    /// Raises the elements of the matrix to the given exponent in-place.
    ///
    /// # Parameters
    ///
    /// - `exponent`: The exponent to which each element of the matrix should be raised.
    ///
    /// # Example:
    ///
    /// ```
    /// use mathematica::Matrix;
    ///
    /// fn main() {
    ///     let mut matrix = Matrix::new([
    ///         [1_f64, 2_f64],
    ///         [3_f64, 4_f64]
    ///     ]);
    ///
    ///     let exponent = 3_f64;
    ///     let power_matrix = matrix.power(exponent);
    ///
    ///     println!("Matrix:\n{:?}\nPower Matrix:\n{:?}", matrix, power_matrix);
    /// }
    ///
    /// /*
    ///     Outputs the Following:
    ///     ----------------------
    ///     Matrix:
    ///     [1, 2]
    ///     [3, 4]
    ///     Power Matrix:
    ///     [1, 8]
    ///     [27, 64]
    /// */
    ///
    /// ```
    /// <hr/>
    pub fn power(&mut self, exponent: f64) {
        for i in 0..ROWS {
            for j in 0..COLS {
                self[i][j] = self[i][j].powf(exponent);
            }
        }
    }
}

impl<const ROWS: usize, const COLS: usize> Index<usize> for Matrix<ROWS, COLS> {
    type Output = Vec<f64>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const ROWS: usize, const COLS: usize> IndexMut<usize> for Matrix<ROWS, COLS> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<const ROWS: usize, const COLS: usize> fmt::Debug for Matrix<ROWS, COLS> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.0 {
            write!(f, "[")?;
            for (index, val) in row.iter().enumerate() {
                write!(f, "{}", val)?;
                if index < COLS - 1 {
                    write!(f, ", ")?;
                }
            }
            write!(f, "]\n")?;
        }
        Ok(())
    }
}
