use numerilib::special::Beta;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn regularized_incomplete_beta_function_test1() {
        let x = 0.9_f64;
        let z1 = 1_f64;
        let z2 = 2_f64;

        let beta = Beta::regincbeta(z1, z2, x);

        assert_approx_eq!(0.99, beta, 1e-2);
    }

    #[test]
    pub fn regularized_incomplete_beta_function_test2() {
        let x = 0.5_f64;
        let z1 = 3_f64;
        let z2 = 2_f64;

        let beta = Beta::regincbeta(z1, z2, x);

        assert_approx_eq!(5_f64 / 16_f64, beta);
    }

    #[test]
    pub fn regularized_incomplete_beta_function_test3() {
        let x = 0.25_f64;
        let z1 = 9_f64;
        let z2 = 9_f64;

        let beta = Beta::regincbeta(z1, z2, x);

        assert_approx_eq!(106384445_f64 / 8589934592_f64, beta);
    }

    #[test]
    pub fn regularized_incomplete_beta_function_test4() {
        let x = 1_f64 / 7_f64;
        let z1 = 4_f64;
        let z2 = 6_f64;

        let beta = Beta::regincbeta(z1, z2, x);

        assert_approx_eq!(166081_f64 / 5764801_f64, beta);
    }
}
