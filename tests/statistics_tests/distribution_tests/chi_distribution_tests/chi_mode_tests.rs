use numerilib::stats::distr::Chi;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn chi_mode_test1() {
        let df = 5.0;

        let mode = Chi::mode(df);

        assert_approx_eq!(2_f64, mode);
    }

    #[test]
    fn chi_mode_test2() {
        let df = 7.0;

        let mode = Chi::mode(df);

        assert_approx_eq!(2.449489742783178, mode);
    }

    #[test]
    fn chi_mode_test3() {
        let df = 9.0;

        let mode = Chi::mode(df);

        assert_approx_eq!(2.8284271247461903, mode);
    }

    #[test]
    fn chi_mode_test4() {
        let df = 11.0;

        let mode = Chi::mode(df);

        assert_approx_eq!(3.1622776601683795, mode);
    }
}
