use numerilib::stats::distr::Chi2;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn chi_sqaured_cdf_test1() {
        let df = 4.0;
        let lower = 2.0;
        let upper = 5.0;

        let cdf = Chi2::tailcdf(lower, upper, df);

        assert_approx_eq!(0.448461387163882, cdf);
    }

    #[test]
    pub fn chi_sqaured_cdf_test2() {
        let df = 7.0;
        let lower = 3.0;
        let upper = 6.0;

        let cdf = Chi2::tailcdf(lower, upper, df);

        assert_approx_eq!(0.3452528812528149, cdf);
    }

    #[test]
    pub fn chi_sqaured_cdf_test3() {
        let df = 10.0;
        let lower = 4.0;
        let upper = 7.0;

        let cdf = Chi2::tailcdf(lower, upper, df);

        assert_approx_eq!(0.22190202935052264, cdf);
    }

    #[test]
    pub fn chi_sqaured_cdf_test4() {
        let df = 13.0;
        let lower = 5.0;
        let upper = 8.0;

        let cdf = Chi2::tailcdf(lower, upper, df);

        assert_approx_eq!(0.1315928580815623, cdf);
    }
}
