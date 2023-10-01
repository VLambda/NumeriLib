use numerilib::stats::distr::Poisson;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn poisson_skewness_test1() {
        let lambda = 8.0001_f64;

        let skewness = Poisson::skewness(lambda);

        assert_approx_eq!(0.35355118090529836, skewness);
    }

    #[test]
    fn poisson_skewness_test2() {
        let lambda = 10.98_f64;

        let skewness = Poisson::skewness(lambda);

        assert_approx_eq!(0.30178582014172844, skewness);
    }

    #[test]
    fn poisson_skewness_test3() {
        let lambda = 6.1_f64;

        let skewness = Poisson::skewness(lambda);

        assert_approx_eq!(0.404888165089458, skewness);
    }

    #[test]
    fn poisson_skewness_test4() {
        let lambda = 1_f64;

        let skewness = Poisson::skewness(lambda);

        assert_approx_eq!(1.0, skewness);
    }
}
