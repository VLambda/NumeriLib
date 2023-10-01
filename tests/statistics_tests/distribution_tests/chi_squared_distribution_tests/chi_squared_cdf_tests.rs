use numerilib::stats::distr::Chi2;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn chi_sqaured_cdf_test1() {
        let df = 4.0;
        let x = 2.0;

        let cdf = Chi2::cdf(x, df);

        assert_approx_eq!(0.2642411176598513, cdf);
    }

    #[test]
    pub fn chi_sqaured_cdf_test2() {
        let df = 7.0;
        let x = 3.0;

        let cdf = Chi2::cdf(x, df);

        assert_approx_eq!(0.11499776835858849, cdf);
    }

    #[test]
    pub fn chi_sqaured_cdf_test3() {
        let df = 10.0;
        let x = 4.0;

        let cdf = Chi2::cdf(x, df);

        assert_approx_eq!(0.05265301734462204, cdf);
    }

    #[test]
    pub fn chi_sqaured_cdf_test4() {
        let df = 13.0;
        let x = 5.0;

        let cdf = Chi2::cdf(x, df);

        assert_approx_eq!(0.024806866676433878, cdf);
    }
}
