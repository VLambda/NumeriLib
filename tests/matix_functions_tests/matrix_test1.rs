use numerilib::Matrix;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn matrix_test() {
        let matrix = Matrix::new([[1_f64, 2_f64], [3_f64, 4_f64]]);

        assert_approx_eq!(1_f64, &matrix.get(1, 1));
        assert_approx_eq!(2_f64, &matrix.get(1, 2));
        assert_approx_eq!(3_f64, &matrix.get(2, 1));
        assert_approx_eq!(4_f64, &matrix.get(2, 2));
    }

    #[test]
    pub fn matrix_update_test() {
        let mut matrix = Matrix::new([[1_f64, 2_f64], [3_f64, 4_f64]]);

        matrix.update(2, 2, 5_f64);

        assert_approx_eq!(5_f64, &matrix.get(2, 2));
    }

    #[test]
    pub fn matrix_determinant_test() {
        let matrix = Matrix::new([[1_f64, 2_f64], [3_f64, 4_f64]]);

        assert_approx_eq!(-2_f64, matrix.determinant());
    }

    #[test]
    pub fn matrix_transpose_test() {
        let matrix = Matrix::new([[1_f64, 2_f64], [3_f64, 4_f64]]);

        let transpose = matrix.transpose();

        assert_approx_eq!(1_f64, &transpose.get(1, 1));
        assert_approx_eq!(3_f64, &transpose.get(1, 2));
        assert_approx_eq!(2_f64, &transpose.get(2, 1));
        assert_approx_eq!(4_f64, &transpose.get(2, 2));
    }

    #[test]
    pub fn matrix_dimensions_test() {
        let matrix = Matrix::new([[1_f64, 2_f64], [3_f64, 4_f64]]);
        let dimensions = matrix.dimensions();

        assert_eq!(2, dimensions[0]);
        assert_eq!(2, dimensions[1]);
    }

    #[test]
    pub fn identity_matrix_test() {
        let identity_matrix = Matrix::<2, 2>::identity();

        assert_eq!(1_f64, identity_matrix.get(1, 1));
        assert_eq!(0_f64, identity_matrix.get(1, 2));
        assert_eq!(0_f64, identity_matrix.get(2, 1));
        assert_eq!(1_f64, identity_matrix.get(2, 2));
    }

    #[test]
    pub fn cum_sum_rows_matrix_test() {
        let matrix = Matrix::new([[1_f64, 2_f64], [3_f64, 4_f64]]);

        let cum_sum_rows_matrix = matrix.cumsumr();

        assert_approx_eq!(1_f64, &cum_sum_rows_matrix.get(1, 1));
        assert_approx_eq!(3_f64, &cum_sum_rows_matrix.get(1, 2));
        assert_approx_eq!(3_f64, &cum_sum_rows_matrix.get(2, 1));
        assert_approx_eq!(7_f64, &cum_sum_rows_matrix.get(2, 2));
    }

    #[test]
    pub fn cum_sum_cols_matrix_test() {
        let matrix = Matrix::new([[1_f64, 2_f64], [3_f64, 4_f64]]);

        let cum_sum_cols_matrix = matrix.cumsumc();

        assert_approx_eq!(1_f64, &cum_sum_cols_matrix.get(1, 1));
        assert_approx_eq!(2_f64, &cum_sum_cols_matrix.get(1, 2));
        assert_approx_eq!(4_f64, &cum_sum_cols_matrix.get(2, 1));
        assert_approx_eq!(6_f64, &cum_sum_cols_matrix.get(2, 2));
    }

    #[test]
    pub fn matrix_arithimatic_addition_test() {
        let matrix1 = Matrix::new([[1_f64, 2_f64], [3_f64, 4_f64]]);

        let matrix2 = Matrix::new([[5_f64, 6_f64], [7_f64, 8_f64]]);

        let matrix_addition = Matrix::arithmetic(&matrix1, &matrix2, |a, b| a + b);

        assert_approx_eq!(6_f64, &matrix_addition.get(1, 1));
        assert_approx_eq!(8_f64, &matrix_addition.get(1, 2));
        assert_approx_eq!(10_f64, &matrix_addition.get(2, 1));
        assert_approx_eq!(12_f64, &matrix_addition.get(2, 2));
    }

    #[test]
    pub fn matrix_arithmetic_subtraction_test() {
        let matrix1 = Matrix::new([[1_f64, 2_f64], [3_f64, 4_f64]]);

        let matrix2 = Matrix::new([[5_f64, 6_f64], [7_f64, 8_f64]]);

        let matrix_subtraction = Matrix::arithmetic(&matrix1, &matrix2, |a, b| a - b);

        assert_approx_eq!(-4_f64, &matrix_subtraction.get(1, 1));
        assert_approx_eq!(-4_f64, &matrix_subtraction.get(1, 2));
        assert_approx_eq!(-4_f64, &matrix_subtraction.get(2, 1));
        assert_approx_eq!(-4_f64, &matrix_subtraction.get(2, 2));
    }

    #[test]
    pub fn matrix_arithmetic_multiplication_test() {
        let matrix1 = Matrix::new([[1_f64, 2_f64], [3_f64, 4_f64]]);

        let matrix2 = Matrix::new([[5_f64, 6_f64], [7_f64, 8_f64]]);

        let matrix_multiplication = Matrix::arithmetic(&matrix1, &matrix2, |a, b| a * b);

        assert_approx_eq!(5_f64, &matrix_multiplication.get(1, 1));
        assert_approx_eq!(12_f64, &matrix_multiplication.get(1, 2));
        assert_approx_eq!(21_f64, &matrix_multiplication.get(2, 1));
        assert_approx_eq!(32_f64, &matrix_multiplication.get(2, 2));
    }

    #[test]
    pub fn matrix_arithmetic_division_test() {
        let matrix1 = Matrix::new([[1_f64, 2_f64], [3_f64, 4_f64]]);

        let matrix2 = Matrix::new([[5_f64, 6_f64], [7_f64, 8_f64]]);

        let matrix_division = Matrix::arithmetic(&matrix1, &matrix2, |a, b| a / b);

        assert_approx_eq!(0.2_f64, &matrix_division.get(1, 1));
        assert_approx_eq!(0.3333333333333333_f64, &matrix_division.get(1, 2));
        assert_approx_eq!(0.42857142857142855_f64, &matrix_division.get(2, 1));
        assert_approx_eq!(0.5_f64, &matrix_division.get(2, 2));
    }

    #[test]
    pub fn matrix_arithmetic_power_test_power_test() {
        let matrix1 = Matrix::new([[1_f64, 2_f64], [3_f64, 4_f64]]);

        let matrix2 = Matrix::new([[5_f64, 6_f64], [7_f64, 8_f64]]);

        let matrix_power = Matrix::arithmetic(&matrix1, &matrix2, |a, b| a.powf(b));

        assert_approx_eq!(1_f64, &matrix_power.get(1, 1));
        assert_approx_eq!(64_f64, &matrix_power.get(1, 2));
        assert_approx_eq!(2187_f64, &matrix_power.get(2, 1));
        assert_approx_eq!(65536_f64, &matrix_power.get(2, 2));
    }

    #[test]
    pub fn matrix_power_test() {
        let mut matrix = Matrix::new([[1_f64, 2_f64], [3_f64, 4_f64]]);

        matrix.power(2_f64);

        assert_approx_eq!(1_f64, &matrix.get(1, 1));
        assert_approx_eq!(4_f64, &matrix.get(1, 2));
        assert_approx_eq!(9_f64, &matrix.get(2, 1));
        assert_approx_eq!(16_f64, &matrix.get(2, 2));
    }
}
