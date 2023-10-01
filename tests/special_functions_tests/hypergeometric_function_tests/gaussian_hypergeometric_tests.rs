use numerilib::special::Hypergeometric;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn gaussian_hypergeometric_function_test1() {
        let a = 1_f64;
        let b = 2_f64;
        let c = 3_f64;
        let x = 0.5_f64;

        let hypergeometric = Hypergeometric::gaussian(a, b, c, x);

        assert_approx_eq!(1.5451774444747588, hypergeometric);
    }

    #[test]
    pub fn gaussian_hypergeometric_function_test2() {
        let a = 5_f64;
        let b = 6_f64;
        let c = 7_f64;
        let x = 0.1_f64;

        let hypergeometric = Hypergeometric::gaussian(a, b, c, x);

        assert_approx_eq!(1.569414715876685, hypergeometric);
    }

    #[test]
    pub fn gaussian_hypergeometric_function_test3() {
        let a = 8_f64;
        let b = 9_f64;
        let c = 10_f64;
        let x = 0.2_f64;

        let hypergeometric = Hypergeometric::gaussian(a, b, c, x);

        assert_approx_eq!(4.970262146513372, hypergeometric);
    }

    #[test]
    pub fn gaussian_hypergeometric_function_test4() {
        let a = 70_f64;
        let b = 40_f64;
        let c = 8_f64;
        let x = 1_f64 - std::f64::consts::LN_2;

        let hypergeometric = Hypergeometric::gaussian(a, b, c, x);

        assert_approx_eq!(7.789026725721121e26, hypergeometric);
    }
}
