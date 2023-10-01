use numerilib::special::Gamma;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn complimentary_incomplete_gamma_function_test1() {
        let bound = 3_f64;
        let x = 1_f64;

        let incgamma = Gamma::incgammac(bound, x);

        assert_approx_eq!(1.8393972058294297, incgamma);
    }

    #[test]
    pub fn complimentary_incomplete_gamma_function_test2() {
        let bound = 3_f64;
        let x = 2_f64;

        let incgamma = Gamma::incgammac(bound, x);

        assert_approx_eq!(1.35335283233834464, incgamma);
    }

    #[test]
    pub fn complimentary_incomplete_gamma_function_test3() {
        let bound = 3_f64;
        let x = 3_f64;

        let incgamma = Gamma::incgammac(bound, x);

        assert_approx_eq!(0.8463801622259044, incgamma);
    }

    #[test]
    pub fn complimentary_incomplete_gamma_function_test4() {
        let bound = 3_f64;
        let x = 4_f64;

        let incgamma = Gamma::incgammac(bound, x);

        assert_approx_eq!(0.4762066110793046, incgamma);
    }
}
