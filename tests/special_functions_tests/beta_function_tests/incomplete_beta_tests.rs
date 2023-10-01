use numerilib::special::Beta;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn incomplete_beta_function_test1() {
        let x = 0.5_f64;
        let z1 = 3_f64;
        let z2 = 2_f64;

        let beta = Beta::incbeta(z1, z2, x);

        assert_approx_eq!(5_f64 / 192_f64, beta);
    }

    #[test]
    pub fn incomplete_beta_function_test2() {
        let x = 0.25_f64;
        let z1 = 7_f64;
        let z2 = 9_f64;

        let beta = Beta::incbeta(z1, z2, x);

        assert_approx_eq!(1.256e-6, beta);
    }

    #[test]
    pub fn incomplete_beta_function_test3() {
        let x = 1_f64 / 7_f64;
        let z1 = 8_f64;
        let z2 = 9_f64;

        let beta = Beta::incbeta(z1, z2, x);

        assert_approx_eq!(25229196673_f64 / 3421662531446118960_f64, beta);
    }

    #[test]
    pub fn incomplete_beta_function_test4() {
        let x = 1_f64 / 3_f64;
        let z1 = 1_f64;
        let z2 = 2_f64;

        let beta = Beta::incbeta(z1, z2, x);

        assert_approx_eq!(5_f64 / 18_f64, beta);
    }
}
