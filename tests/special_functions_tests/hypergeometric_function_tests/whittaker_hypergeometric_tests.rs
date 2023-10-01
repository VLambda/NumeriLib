use numerilib::special::Hypergeometric;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn whittaker_hypergeometric_function_test1() {
        let k = 1_f64;
        let m = 2_f64;
        let z = 0.5_f64;

        let hypergeometric = Hypergeometric::whittaker(k, m, z);

        assert_approx_eq!(0.1606687379402003, hypergeometric);
    }

    #[test]
    pub fn whittaker_hypergeometric_function_test2() {
        let k = 5_f64;
        let m = 6_f64;
        let z = 0.1_f64;

        let hypergeometric = Hypergeometric::whittaker(k, m, z);

        assert_approx_eq!(0.0000000000000001, hypergeometric);
    }

    #[test]
    pub fn whittaker_hypergeometric_function_test3() {
        let k = 8_f64;
        let m = 9_f64;
        let z = 0.2_f64;

        let hypergeometric = Hypergeometric::whittaker(k, m, z);

        assert_approx_eq!(0.0000000000000001, hypergeometric);
    }

    #[test]
    pub fn whittaker_hypergeometric_function_test4() {
        let k = 5_f64;
        let m = 6_f64;
        let z = 0.4;

        let hypergeometric = Hypergeometric::whittaker(k, m, z);

        assert_approx_eq!(0.0022224511347866786, hypergeometric);
    }
}
