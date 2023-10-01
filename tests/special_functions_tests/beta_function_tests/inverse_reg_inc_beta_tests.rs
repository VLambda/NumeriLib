use numerilib::special::Beta;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn inv_reg_inc_beta_function_test1() {
        let x = 0.99_f64;
        let z1 = 1_f64;
        let z2 = 2_f64;

        let beta = Beta::invregincbeta(z1, z2, x);

        assert_approx_eq!(0.9, beta, 1e-2);
    }

    #[test]
    pub fn inv_reg_inc_beta_function_test2() {
        let x = 5_f64 / 16_f64;
        let z1 = 3_f64;
        let z2 = 2_f64;

        let beta = Beta::invregincbeta(z1, z2, x);

        assert_approx_eq!(0.5, beta);
    }

    #[test]
    pub fn inv_reg_inc_beta_function_test3() {
        let x = 106384445_f64 / 8589934592_f64;
        let z1 = 9_f64;
        let z2 = 9_f64;

        let beta = Beta::invregincbeta(z1, z2, x);

        assert_approx_eq!(0.25, beta);
    }

    #[test]
    pub fn inv_reg_inc_beta_function_test4() {
        let x = 166081_f64 / 5764801_f64;
        let z1 = 4_f64;
        let z2 = 6_f64;

        let beta = Beta::invregincbeta(z1, z2, x);

        assert_approx_eq!(1_f64 / 7_f64, beta);
    }
}
