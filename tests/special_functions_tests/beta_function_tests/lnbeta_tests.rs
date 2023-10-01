use numerilib::special::Beta;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn log_beta_function_test1() {
        let z1 = 1_f64;
        let z2 = 2_f64;

        let beta = Beta::lnbeta(z1, z2);

        assert_approx_eq!(-std::f64::consts::LN_2, beta);
    }

    #[test]
    pub fn log_beta_function_test2() {
        let z1 = 2_f64;
        let z2 = 2_f64;

        let beta = Beta::lnbeta(z1, z2);

        assert_approx_eq!(-6_f64.ln(), beta);
    }

    #[test]
    pub fn log_beta_function_test3() {
        let z1 = 8_f64;
        let z2 = 7_f64;

        let beta = Beta::lnbeta(z1, z2);

        assert_approx_eq!(-10.08680860968683, beta);
    }

    #[test]
    pub fn log_beta_function_test4() {
        let z1 = 8_f64;
        let z2 = 5_f64;

        let beta = Beta::lnbeta(z1, z2);

        assert_approx_eq!(-8.283999304269042, beta);
    }
}
