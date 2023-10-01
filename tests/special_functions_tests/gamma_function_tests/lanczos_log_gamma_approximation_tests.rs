use numerilib::special::Gamma;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn log_gamma_function_test1() {
        let x = 1_f64;

        let gamma = Gamma::lanczosln(x);

        assert_approx_eq!(0_f64, gamma);
    }

    #[test]
    pub fn log_gamma_function_test2() {
        let x = 2_f64;

        let gamma = Gamma::lanczosln(x);

        assert_approx_eq!(0_f64, gamma);
    }

    #[test]
    pub fn log_gamma_function_test3() {
        let x = 3_f64;

        let gamma = Gamma::lanczosln(x);

        assert_approx_eq!(std::f64::consts::LN_2, gamma);
    }

    #[test]
    pub fn log_gamma_function_test4() {
        let x = 4_f64;

        let gamma = Gamma::lanczosln(x);

        assert_approx_eq!(6_f64.ln(), gamma);
    }
}
