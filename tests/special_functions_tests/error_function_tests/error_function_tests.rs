use numerilib::special::Error;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn error_function_test1() {
        let z = 0.5_f64;

        let erf = Error::erf(z);

        assert_approx_eq!(0.5204998778130465, erf);
    }

    #[test]
    pub fn error_function_test2() {
        let z = 1.0_f64;

        let erf = Error::erf(z);

        assert_approx_eq!(0.8427007929497148, erf);
    }

    #[test]
    pub fn error_function_test3() {
        let z = 1.5_f64;

        let erf = Error::erf(z);

        assert_approx_eq!(0.9661051464753451, erf);
    }

    #[test]
    pub fn error_function_test4() {
        let z = 2.0_f64;

        let erf = Error::erf(z);

        assert_approx_eq!(0.9953222650189527, erf);
    }
}
