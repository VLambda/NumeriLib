use numerilib::stats::distr::Poisson;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn poisson_mode_test1() {
        let lambda = 8.0001_f64;

        let mode = Poisson::mode(lambda);

        assert_approx_eq!(8.0, mode);
    }

    #[test]
    fn poisson_mode_test2() {
        let lambda = 10.98_f64;

        let mode = Poisson::mode(lambda);

        assert_approx_eq!(10.0, mode);
    }

    #[test]
    fn poisson_mode_test3() {
        let lambda = 6.1_f64;

        let mode = Poisson::mode(lambda);

        assert_approx_eq!(6.0, mode);
    }

    #[test]
    fn poisson_mode_test4() {
        let lambda = 1_f64;

        let mode = Poisson::mode(lambda);

        assert_approx_eq!(1.0, mode);
    }
}
