use numerilib::stats::distr::Fisher;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn fisher_mode_test1() {
        let d1 = 4_f64;
        let d2 = 8_f64;

        let mode = Fisher::mode(d1, d2);

        assert_approx_eq!(0.4, mode);
    }

    #[test]
    pub fn fisher_mode_test2() {
        let d1 = 8_f64;
        let d2 = 4_f64;

        let mode = Fisher::mode(d1, d2);

        assert_approx_eq!(0.5, mode);
    }

    #[test]
    pub fn fisher_mode_test3() {
        let d1 = 7_f64;
        let d2 = 10_f64;

        let mode = Fisher::mode(d1, d2);

        assert_approx_eq!(0.5952380952380952, mode);
    }

    #[test]
    pub fn fisher_mode_test4() {
        let d1 = 16_f64;
        let d2 = 2_f64;

        let mode = Fisher::mode(d1, d2);

        assert_approx_eq!(0.4375, mode);
    }
}
