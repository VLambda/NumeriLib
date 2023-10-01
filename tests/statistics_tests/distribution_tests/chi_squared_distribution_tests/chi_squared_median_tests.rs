use numerilib::stats::distr::Chi2;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn chi_sqaured_median_test1() {
        let df = 4.0;

        let cdf = Chi2::median(df);

        assert_approx_eq!(3.3696844993141286, cdf);
    }

    #[test]
    pub fn chi_sqaured_median_test2() {
        let df = 7.0;

        let cdf = Chi2::median(df);

        assert_approx_eq!(6.354273396601439, cdf);
    }

    #[test]
    pub fn chi_sqaured_median_test3() {
        let df = 10.0;

        let cdf = Chi2::median(df);

        assert_approx_eq!(9.34803840877915, cdf);
    }

    #[test]
    pub fn chi_sqaured_median_test4() {
        let df = 13.0;

        let cdf = Chi2::median(df);

        assert_approx_eq!(12.344664410191474, cdf);
    }
}
