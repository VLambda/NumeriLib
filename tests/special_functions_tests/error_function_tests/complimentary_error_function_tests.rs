use numerilib::special::Error;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn complimentary_error_function_test1() {
        let z = 0.5_f64;

        let erfc = Error::erfc(z);

        assert_approx_eq!(0.4795001221869535_f64, erfc);
    }

    #[test]
    pub fn complimentary_error_function_test2() {
        let z = 1.0_f64;

        let erfc = Error::erfc(z);

        assert_approx_eq!(0.1572992070502851_f64, erfc);
    }

    #[test]
    pub fn complimentary_error_function_test3() {
        let z = 1.5_f64;

        let erfc = Error::erfc(z);

        assert_approx_eq!(0.0338948535246549_f64, erfc);
    }

    #[test]
    pub fn complimentary_error_function_test4() {
        let z = 2.0_f64;

        let erfc = Error::erfc(z);

        assert_approx_eq!(0.0046777349810473_f64, erfc);
    }
}
