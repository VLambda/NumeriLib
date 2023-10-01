use numerilib::stats::distr::Chi;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn chi_cdf_test1() {
        let df = 5.0;
        let x = 2.0;

        let cdf = Chi::cdf(x, df);

        assert_approx_eq!(0.45058404865277163, cdf);
    }

    #[test]
    fn chi_cdf_test2() {
        let df = 5.0;
        let x = 2.5;

        let cdf = Chi::cdf(x, df);

        assert_approx_eq!(0.717352703414772, cdf);
    }

    #[test]
    fn chi_cdf_test3() {
        let df = 7.0;
        let x = 6.0;

        let cdf = Chi::cdf(x, df);

        assert_approx_eq!(0.9999926565997873, cdf);
    }

    #[test]
    fn chi_cdf_test4() {
        let df = 7.0;
        let x = 0.4;

        let cdf = Chi::cdf(x, df);

        assert_approx_eq!(1.1700160153932344e-5, cdf);
    }
}
