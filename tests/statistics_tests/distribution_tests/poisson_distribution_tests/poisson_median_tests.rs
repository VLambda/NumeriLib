use numerilib::stats::distr::Poisson;

#[cfg(test)]
pub mod test {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn poisson_median_test1() {
        let lambda = 8_f64;

        let median = Poisson::median(lambda);

        assert_approx_eq!(8.0, median);
    }

    #[test]
    fn poisson_median_test2() {
        let lambda = 10_f64;

        let median = Poisson::median(lambda);

        assert_approx_eq!(10.0, median);
    }

    #[test]
    fn poisson_median_test3() {
        let lambda = 6_f64;

        let median = Poisson::median(lambda);

        assert_approx_eq!(6.0, median);
    }

    #[test]
    fn poisson_median_test4() {
        let lambda = 1_f64;

        let median = Poisson::median(lambda);

        assert_approx_eq!(1.0, median);
    }
}
