use numerilib::stats::distr::Chi2;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn chi_sqaured_variance_test1() {
        let df = 4.0;

        let variance = Chi2::variance(df);

        assert_approx_eq!(8_f64, variance);
    }

    #[test]
    pub fn chi_sqaured_variance_test2() {
        let df = 7.0;

        let variance = Chi2::variance(df);

        assert_approx_eq!(14_f64, variance);
    }

    #[test]
    pub fn chi_sqaured_variance_test3() {
        let df = 10.0;

        let variance = Chi2::variance(df);

        assert_approx_eq!(20_f64, variance);
    }

    #[test]
    pub fn chi_sqaured_variance_test4() {
        let df = 13.0;

        let variance = Chi2::variance(df);

        assert_approx_eq!(26_f64, variance);
    }
}
