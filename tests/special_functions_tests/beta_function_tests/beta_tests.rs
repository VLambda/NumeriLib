use numerilib::special::Beta;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn beta_function_test1() {
        let z1 = 1_f64;
        let z2 = 2_f64;

        let beta = Beta::beta(z1, z2);

        assert_approx_eq!(0.5_f64, beta);
    }

    #[test]
    pub fn beta_function_test2() {
        let z1 = 2_f64;
        let z2 = 2_f64;

        let beta = Beta::beta(z1, z2);

        assert_approx_eq!(1_f64 / 6_f64, beta);
    }

    #[test]
    pub fn beta_function_test3() {
        let z1 = 8_f64;
        let z2 = 7_f64;

        let beta = Beta::beta(z1, z2);

        assert_approx_eq!(1_f64 / 24024_f64, beta);
    }

    #[test]
    pub fn beta_function_test4() {
        let z1 = 8_f64;
        let z2 = 5_f64;

        let beta = Beta::beta(z1, z2);

        assert_approx_eq!(1_f64 / 3960_f64, beta);
    }
}
