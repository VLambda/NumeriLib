use numerilib::special::Gamma;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn regularized_incomplete_gamma_function_test1() {
        let bound = 2_f64;
        let x = 1_f64;

        let incgamma = Gamma::reggamma(bound, x);

        assert_approx_eq!(0.2642411176598513, incgamma);
    }

    #[test]
    pub fn regularized_incomplete_gamma_function_test2() {
        let bound = 6_f64;
        let x = 7_f64;

        let incgamma = Gamma::reggamma(bound, x);

        assert_approx_eq!(0.6992917238357631, incgamma);
    }

    #[test]
    pub fn regularized_incomplete_gamma_function_test3() {
        let bound = 9_f64;
        let x = 3_f64;

        let incgamma = Gamma::reggamma(bound, x);

        assert_approx_eq!(0.0038029920617433394, incgamma);
    }

    #[test]
    pub fn regularized_incomplete_gamma_function_test4() {
        let bound = 1_f64;
        let x = 1_f64;

        let incgamma = Gamma::reggamma(bound, x);

        assert_approx_eq!(0.6321205588318652, incgamma);
    }
}
