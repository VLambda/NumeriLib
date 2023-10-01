use numerilib::special::Gamma;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn complimentary_regularized_incomplete_gamma_function_test1() {
        let bound = 2_f64;
        let x = 1_f64;

        let incgamma = Gamma::reggammac(bound, x);

        assert_approx_eq!(0.7357588823401486, incgamma);
    }

    #[test]
    pub fn complimentary_regularized_incomplete_gamma_function_test2() {
        let bound = 6_f64;
        let x = 7_f64;

        let incgamma = Gamma::reggammac(bound, x);

        assert_approx_eq!(0.3007082761642369, incgamma);
    }

    #[test]
    pub fn complimentary_regularized_incomplete_gamma_function_test3() {
        let bound = 9_f64;
        let x = 3_f64;

        let incgamma = Gamma::reggammac(bound, x);

        assert_approx_eq!(0.9961970079382566, incgamma);
    }

    #[test]
    pub fn complimentary_regularized_incomplete_gamma_function_test4() {
        let bound = 1_f64;
        let x = 1_f64;

        let incgamma = Gamma::reggammac(bound, x);

        assert_approx_eq!(0.36787944116813476, incgamma);
    }
}
