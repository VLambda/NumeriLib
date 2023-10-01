use numerilib::stats::distr::Chi2;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn chi_sqaured_mode_test1() {
        let df = 4.0;

        let mode = Chi2::mode(df);

        assert_approx_eq!(2_f64, mode);
    }

    #[test]
    pub fn chi_sqaured_mode_test2() {
        let df = 7.0;

        let mode = Chi2::mode(df);

        assert_approx_eq!(5_f64, mode);
    }

    #[test]
    pub fn chi_sqaured_mode_test3() {
        let df = 10.0;

        let mode = Chi2::mode(df);

        assert_approx_eq!(8_f64, mode);
    }

    #[test]
    pub fn chi_sqaured_mode_test4() {
        let df = 13.0;

        let mode = Chi2::mode(df);

        assert_approx_eq!(11_f64, mode);
    }
}
