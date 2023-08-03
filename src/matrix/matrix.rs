pub struct Matrix;

impl Matrix {
    /// Creates a Matrix <br>
    /// Learn more at: <a href="https://wikipedia.org/wiki/Matrix_(mathematics)" target="_blank">Wikipedia Matrix</a> <br>
    /// <hr/>
    ///
    /// # Example
    ///
    /// ```
    /// use ferrate::Matrix;
    ///
    /// let rows = 2;
    /// let columns = 3;
    /// let default_value = 3_f64;
    ///
    /// let matrix = Matrix::create(rows, columns, default_value);
    ///
    /// /* Creates Matrix
    /// [3.0,3.0,3.0]
    /// [3.0,3.0,3.0]
    /// */
    /// ```
    /// <hr/>
    pub fn create(rows: usize, columns: usize, default_value: f64) -> Vec<Vec<f64>> {
        vec![vec![default_value; columns]; rows]
    }
    /// Updates the Values in a Matrix <br>
    /// Learn more at: <a href="https://wikipedia.org/wiki/Matrix_(mathematics)" target="_blank">Wikipedia Matrix</a> <br>
    /// <hr/>
    ///
    /// # Example:
    ///
    /// ```
    /// use ferrate::Matrix;
    ///
    /// let rows = 2;
    /// let columns = 3;
    /// let default_value = 3_f64;
    /// let mut matrix = Matrix::create(rows, columns, default_value);
    ///
    /// Matrix::update(&mut matrix, 1, 1, 42_f64);
    ///
    /// /* Updates the Matrix
    /// Before:
    /// [3.0,3.0,3.0]
    /// [3.0,3.0,3.0]
    ///
    /// After:
    /// [3.0,3.0,3.0]
    /// [3.0,42.0,3.0]
    /// */
    ///
    /// ```
    /// <hr/>
    pub fn update(matrix: &mut Vec<Vec<f64>>, row: usize, column: usize, new_value: f64) {
        if let Some(row_vec) = matrix.get_mut(row) {
            if let Some(element) = row_vec.get_mut(column) {
                *element = new_value;
            }
        }
    }
    /// Calculates the Determinate of a square Matrix with the Leibniz formula <br>
    /// Learn about determinate at: <a href="https://wikipedia.org/wiki/Determinant" target="_blank">Wikipedia Determinate</a> <br>
    /// Learn about the Leibniz formula at: <a href="https://wikipedia.org/wiki/Leibniz_formula_for_determinants" target="_blank">Wikipedia Leibniz Formula</a> <br>
    /// Learn more about Square Matrix's at: <a href="https://wikipedia.org/wiki/Square_matrix" target="_blank">Wikipedia Square Matrix</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example:
    ///
    /// ```
    /// use ferrate::Matrix;
    ///
    /// let rows = 4;
    /// let columns = 4;
    /// let default_value = 0_f64;
    ///
    /// let mut matrix = Matrix::create(rows, columns, default_value);
    ///
    /// Matrix::update(&mut matrix, 0, 0, 4_f64);
    /// Matrix::update(&mut matrix, 0, 1, 5_f64);
    /// Matrix::update(&mut matrix, 0, 2, 6_f64);
    /// Matrix::update(&mut matrix, 0, 3, 7_f64);
    /// Matrix::update(&mut matrix, 1, 0, 3_f64);
    /// Matrix::update(&mut matrix, 1, 1, 42_f64);
    /// Matrix::update(&mut matrix, 1, 2, 2_f64);
    /// Matrix::update(&mut matrix, 1, 3, 1_f64);
    /// Matrix::update(&mut matrix, 2, 0, 2_f64);
    /// Matrix::update(&mut matrix, 2, 1, 3_f64);
    /// Matrix::update(&mut matrix, 2, 2, 17_f64);
    /// Matrix::update(&mut matrix, 2, 3, 52_f64);
    /// Matrix::update(&mut matrix, 3, 0, 8_f64);
    /// Matrix::update(&mut matrix, 3, 1, 7_f64);
    /// Matrix::update(&mut matrix, 3, 2, 9_f64);
    /// Matrix::update(&mut matrix, 3, 3, 5_f64);
    ///
    /// let determinate = Matrix::determinant(&matrix);
    ///
    /// /* Calculates the Determinate of this Matrix:
    /// [4,5,6,7]
    /// [3,42,2,1]
    /// [2,3,17,52]
    /// [8,7,9,5]
    /// */
    ///
    /// assert_eq!(determinate, 3705_f64);
    /// ```
    /// <hr/>
    pub fn determinant(matrix: &Vec<Vec<f64>>) -> f64 {
        let n = matrix.len();
        if n != matrix[0].len() {
            panic!("Matrix is not square!");
        }

        if n == 1 {
            return matrix[0][0];
        }

        let mut sum = 0.0;
        let mut sign = 1.0;

        for i in 0..n {
            let submatrix: Vec<Vec<f64>> = matrix
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

            sum += sign * matrix[i][0] * Self::determinant(&submatrix);
            sign *= -1.0;
        }

        sum
    }
    /// The Transpose Function in Rust <br>
    /// Learn more at: <a href="https://wikipedia.org/wiki/Transpose" target="_blank">Wikipedia Transpose</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example:
    ///
    /// ```
    /// use ferrate::Matrix;
    ///
    /// let rows = 4;
    /// let columns = 4;
    /// let default_value = 0_f64;
    ///
    /// let mut matrix = Matrix::create(rows, columns, default_value);
    ///
    /// Matrix::update(&mut matrix, 0, 0, 4_f64);
    /// Matrix::update(&mut matrix, 0, 1, 5_f64);
    /// Matrix::update(&mut matrix, 0, 2, 6_f64);
    /// Matrix::update(&mut matrix, 0, 3, 7_f64);
    /// Matrix::update(&mut matrix, 1, 0, 3_f64);
    /// Matrix::update(&mut matrix, 1, 1, 42_f64);
    /// Matrix::update(&mut matrix, 1, 2, 2_f64);
    /// Matrix::update(&mut matrix, 1, 3, 1_f64);
    /// Matrix::update(&mut matrix, 2, 0, 2_f64);
    /// Matrix::update(&mut matrix, 2, 1, 3_f64);
    /// Matrix::update(&mut matrix, 2, 2, 17_f64);
    /// Matrix::update(&mut matrix, 2, 3, 52_f64);
    /// Matrix::update(&mut matrix, 3, 0, 8_f64);
    /// Matrix::update(&mut matrix, 3, 1, 7_f64);
    /// Matrix::update(&mut matrix, 3, 2, 9_f64);
    /// Matrix::update(&mut matrix, 3, 3, 5_f64);
    ///
    /// Matrix::transpose(&matrix);
    ///
    /// /* Transposes the Matrix:
    /// Before:
    /// [4,5,6,7]
    /// [3,42,2,1]
    /// [2,3,17,52]
    /// [8,7,9,5]
    /// After:
    /// [4,3,2,8]
    /// [5,42,3,7]
    /// [6,2,17,9]
    /// [7,1,52,5]
    /// */
    /// ```
    /// <hr/>
    pub fn transpose(matrix: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
        let rows = matrix.len();
        let columns = matrix[0].len();
        let mut transposed_matrix: Vec<Vec<f64>> = vec![vec![0.0; rows]; columns];

        for i in 0..rows {
            for j in 0..columns {
                transposed_matrix[j][i] = matrix[i][j];
            }
        }

        transposed_matrix
    }
    /// Returns the Dimensions of a Matrix <br>
    /// Learn more at: <a href="https://wikipedia.org/wiki/Matrix_(mathematics)" target="_blank">Wikipedia Matrix</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example:
    ///
    /// ```
    /// use ferrate::Matrix;
    ///
    /// let rows = 3;
    /// let columns = 4;
    /// let default_value = 0_f64;
    ///
    /// let matrix = Matrix::create(rows, columns, default_value);
    ///
    /// /* The Matrix:
    /// [0,0,0,0]
    /// [0,0,0,0]
    /// [0,0,0,0]
    /// */
    ///
    /// let matrix_dimensions = Matrix::dimensions(&matrix);
    ///
    /// /* Dimensions:
    /// [3,4]
    /// */
    /// ```
    /// <hr/>
    pub fn dimensions(matrix: &Vec<Vec<f64>>) -> Vec<usize> {
        let rows = matrix.len();
        let columns = matrix[0].len();

        vec![rows, columns]
    }
    /// Returns the Identity Matrix of a given size <br>
    /// Learn more at: <a href="https://wikipedia.org/wiki/Identity_matrix" target="_blank">Wikipedia Identity Matrix</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example
    ///
    /// ```
    /// use ferrate::Matrix;
    ///
    /// let size = 5;
    ///
    /// let matrix = Matrix::identity(size);
    ///
    /// /* The 5 x 5 Identity Matrix:
    /// [1,0,0,0,0]
    /// [0,1,0,0,0]
    /// [0,0,1,0,0]
    /// [0,0,0,1,0]
    /// [0,0,0,0,1]
    /// */
    /// ```
    /// <hr/>
    pub fn identity(n: usize) -> Vec<Vec<f64>> {
        let mut identity_matrix = vec![vec![0.0; n]; n];
        for i in 0..n {
            identity_matrix[i][i] = 1.0;
        }
        identity_matrix
    }
    /// Calculates the Cumulative Sum of a Matrix's Rows <br>
    /// Learn more at: <a href="https://www.mathworks.com/help/matlab/ref/cumsum.html#btrgrnv-1-dim" target="_blank">MatLab Cumsum</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example:
    ///
    /// ```
    /// use ferrate::Matrix;
    ///
    /// let rows = 4;                                                 
    /// let columns = 4;                                              
    /// let default_value = 0_f64;                                    
    ///                                                               
    /// let mut matrix = Matrix::create(rows, columns, default_value);
    ///                                                               
    /// Matrix::update(&mut matrix, 0, 0, 4_f64);                     
    /// Matrix::update(&mut matrix, 0, 1, 5_f64);                     
    /// Matrix::update(&mut matrix, 0, 2, 6_f64);                     
    /// Matrix::update(&mut matrix, 0, 3, 7_f64);                     
    /// Matrix::update(&mut matrix, 1, 0, 3_f64);                     
    /// Matrix::update(&mut matrix, 1, 1, 42_f64);                    
    /// Matrix::update(&mut matrix, 1, 2, 2_f64);                     
    /// Matrix::update(&mut matrix, 1, 3, 1_f64);                     
    /// Matrix::update(&mut matrix, 2, 0, 2_f64);                     
    /// Matrix::update(&mut matrix, 2, 1, 3_f64);                     
    /// Matrix::update(&mut matrix, 2, 2, 17_f64);                    
    /// Matrix::update(&mut matrix, 2, 3, 52_f64);                    
    /// Matrix::update(&mut matrix, 3, 0, 8_f64);                     
    /// Matrix::update(&mut matrix, 3, 1, 7_f64);                     
    /// Matrix::update(&mut matrix, 3, 2, 9_f64);                     
    /// Matrix::update(&mut matrix, 3, 3, 5_f64);                     
    ///                                                               
    /// Matrix::cumsumr(&matrix);                                   
    ///                                                               
    /// /* Row CumSum the Matrix:                                     
    /// Before:                                                       
    /// [4,5,6,7]                                                     
    /// [3,42,2,1]                                                    
    /// [2,3,17,52]                                                   
    /// [8,7,9,5]                                                     
    /// After:                                                        
    /// [4,9,15,22]
    /// [3,45,47,48]
    /// [2,5,22,74]
    /// [8,15,24,29]                                                    
    /// */
    /// ```
    /// <hr/>
    pub fn cumsumr(matrix: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
        let mut cumsum_rows_matrix = matrix.clone();
        for row in &mut cumsum_rows_matrix {
            for j in 1..row.len() {
                row[j] += row[j - 1];
            }
        }
        cumsum_rows_matrix
    }
    /// Calculates the Cumulative Sum of a Matrix's Columns <br>
    /// Learn more at: <a href="https://www.mathworks.com/help/matlab/ref/cumsum.html#btrgrnv-1-dim" target="_blank">MatLab Cumsum</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example:
    ///
    /// ```
    /// use ferrate::Matrix;
    ///
    /// let rows = 4;                                                 
    /// let columns = 4;                                              
    /// let default_value = 0_f64;                                    
    ///                                                               
    /// let mut matrix = Matrix::create(rows, columns, default_value);
    ///                                                               
    /// Matrix::update(&mut matrix, 0, 0, 4_f64);                     
    /// Matrix::update(&mut matrix, 0, 1, 5_f64);                     
    /// Matrix::update(&mut matrix, 0, 2, 6_f64);                     
    /// Matrix::update(&mut matrix, 0, 3, 7_f64);                     
    /// Matrix::update(&mut matrix, 1, 0, 3_f64);                     
    /// Matrix::update(&mut matrix, 1, 1, 42_f64);                    
    /// Matrix::update(&mut matrix, 1, 2, 2_f64);                     
    /// Matrix::update(&mut matrix, 1, 3, 1_f64);                     
    /// Matrix::update(&mut matrix, 2, 0, 2_f64);                     
    /// Matrix::update(&mut matrix, 2, 1, 3_f64);                     
    /// Matrix::update(&mut matrix, 2, 2, 17_f64);                    
    /// Matrix::update(&mut matrix, 2, 3, 52_f64);                    
    /// Matrix::update(&mut matrix, 3, 0, 8_f64);                     
    /// Matrix::update(&mut matrix, 3, 1, 7_f64);                     
    /// Matrix::update(&mut matrix, 3, 2, 9_f64);                     
    /// Matrix::update(&mut matrix, 3, 3, 5_f64);                     
    ///                                                               
    /// Matrix::cumsumc(&matrix);                                   
    ///                                                               
    /// /* Column CumSum the Matrix:                                     
    /// Before:                                                       
    /// [4,5,6,7]                                                     
    /// [3,42,2,1]                                                    
    /// [2,3,17,52]                                                   
    /// [8,7,9,5]                                                     
    /// After:                                                        
    /// [4,5,6,7]
    /// [7,47,8,8]
    /// [9,50,25,60]
    /// [17,57,34,65]                                                    
    /// */
    /// ```
    /// <hr/>
    pub fn cumsumc(matrix: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
        let mut cumsum_columns_matrix = Self::transpose(matrix);
        for row in &mut cumsum_columns_matrix {
            for j in 1..row.len() {
                row[j] += row[j - 1];
            }
        }
        Self::transpose(&cumsum_columns_matrix)
    }
    /// An implementation of Gaussian Elimination in Rust <br>
    /// Learn more at: <a href="https://en.wikipedia.org/wiki/Gaussian_elimination" target="_blank">Wikipedia Gaussian Elimination</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example:
    ///
    /// ```
    /// use ferrate::Matrix;                                          
    ///                                                               
    /// let rows = 4;                                                 
    /// let columns = 4;                                              
    /// let default_value = 0_f64;                                    
    ///                                                               
    /// let mut matrix = Matrix::create(rows, columns, default_value);
    ///                                                               
    /// Matrix::update(&mut matrix, 0, 0, 4_f64);                     
    /// Matrix::update(&mut matrix, 0, 1, 5_f64);                     
    /// Matrix::update(&mut matrix, 0, 2, 6_f64);                     
    /// Matrix::update(&mut matrix, 0, 3, 7_f64);                     
    /// Matrix::update(&mut matrix, 1, 0, 3_f64);                     
    /// Matrix::update(&mut matrix, 1, 1, 42_f64);                    
    /// Matrix::update(&mut matrix, 1, 2, 2_f64);                     
    /// Matrix::update(&mut matrix, 1, 3, 1_f64);                     
    /// Matrix::update(&mut matrix, 2, 0, 2_f64);                     
    /// Matrix::update(&mut matrix, 2, 1, 3_f64);                     
    /// Matrix::update(&mut matrix, 2, 2, 17_f64);                    
    /// Matrix::update(&mut matrix, 2, 3, 52_f64);                    
    /// Matrix::update(&mut matrix, 3, 0, 8_f64);                     
    /// Matrix::update(&mut matrix, 3, 1, 7_f64);                     
    /// Matrix::update(&mut matrix, 3, 2, 9_f64);                     
    /// Matrix::update(&mut matrix, 3, 3, 5_f64);                     
    ///                                                               
    /// Matrix::gaussian_elimination(&mut matrix);                                     
    ///                                                               
    /// /* Gaussian Elimination of the Matrix:                                     
    /// Before:                                                       
    /// [4,5,6,7]                                                     
    /// [3,42,2,1]                                                    
    /// [2,3,17,52]                                                   
    /// [8,7,9,5]                                                     
    /// After:                                                        
    /// [1,0,0,0]
    /// [0,1,0,0]
    /// [0,0,1,0]
    /// [0,0,0,1]                                                 
    /// */                                                            
    /// ```
    /// <hr/>
    pub fn gaussian_elimination(matrix: &mut Vec<Vec<f64>>) {
        let rows = matrix.len();
        let columns = matrix[0].len();

        let mut lead = 0;
        for r in 0..rows {
            if lead >= columns {
                break;
            }
            let mut i = r;
            while matrix[i][lead] == 0.0 {
                i += 1;
                if i == rows {
                    i = r;
                    lead += 1;
                    if columns == lead {
                        return;
                    }
                }
            }
            matrix.swap(i, r);
            let lv = matrix[r][lead];
            for j in 0..columns {
                matrix[r][j] /= lv;
            }
            for i in 0..rows {
                if i != r {
                    let lv = matrix[i][lead];
                    for j in 0..columns {
                        matrix[i][j] -= lv * matrix[r][j];
                    }
                }
            }
            lead += 1;
        }
    }
}
