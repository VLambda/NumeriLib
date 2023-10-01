use numerilib::stats::distr::Poisson;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn poisson_cdf_test1() {
        let k = 10_f64;
        let lambda = 8_f64;

        let cdf = Poisson::cdf(k, lambda);

        assert_approx_eq!(0.8158857925532029, cdf);
    }

    #[test]
    fn poisson_cdf_test2() {
        let k = 8_f64;
        let lambda = 10_f64;

        let cdf = Poisson::cdf(k, lambda);

        assert_approx_eq!(0.3328196787507199, cdf);
    }

    #[test]
    fn poisson_cdf_test3() {
        let k = 72_f64;
        let lambda = 6_f64;

        let cdf = Poisson::cdf(k, lambda);

        assert_approx_eq!(0.9999999999999999, cdf);
    }

    #[test]
    fn poisson_cdf_test4() {
        let k = 2_f64;
        let lambda = 1_f64;

        let cdf = Poisson::cdf(k, lambda);

        assert_approx_eq!(0.9196986029274903, cdf);
    }
}
