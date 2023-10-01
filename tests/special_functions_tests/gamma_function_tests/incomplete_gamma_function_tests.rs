use numerilib::special::Gamma;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn incomplete_gamma_function_test1() {
        let bound = 2_f64;
        let x = 1_f64;

        let incgamma = Gamma::incgamma(bound, x);

        assert_approx_eq!(0.2642411176571153, incgamma);
    }

    #[test]
    pub fn incomplete_gamma_function_test2() {
        let bound = 6_f64;
        let x = 7_f64;

        let incgamma = Gamma::incgamma(bound, x);

        assert_approx_eq!(83.91500685878937, incgamma);
    }

    #[test]
    pub fn incomplete_gamma_function_test3() {
        let bound = 9_f64;
        let x = 3_f64;

        let incgamma = Gamma::incgamma(bound, x);

        assert_approx_eq!(153.3366399267751, incgamma);
    }

    #[test]
    pub fn incomplete_gamma_function_test4() {
        let bound = 1_f64;
        let x = 1_f64;

        let incgamma = Gamma::incgamma(bound, x);

        assert_approx_eq!(0.6321205588285577, incgamma);
    }
}
