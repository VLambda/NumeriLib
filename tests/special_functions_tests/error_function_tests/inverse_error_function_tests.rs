use numerilib::special::Error;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn inverse_error_function_test1() {
        let z = 0.5204998778130465_f64;

        let inverf = Error::inverf(z);

        assert_approx_eq!(0.5, inverf);
    }

    #[test]
    pub fn inverse_error_function_test2() {
        let z = 0.8427007929497148_f64;

        let inverf = Error::inverf(z);

        assert_approx_eq!(1_f64, inverf);
    }

    #[test]
    pub fn inverse_error_function_test3() {
        let z = 0.9661051464753451_f64;

        let inverf = Error::inverf(z);

        assert_approx_eq!(1.5, inverf);
    }

    #[test]
    pub fn inverse_error_function_test4() {
        let z = 0.9953222650189527_f64;

        let inverf = Error::inverf(z);

        assert_approx_eq!(2_f64, inverf);
    }
}
