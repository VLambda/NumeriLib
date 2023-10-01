use numerilib::stats::distr::Poisson;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn poisson_sd_test1() {
        let lambda = 8.0001_f64;

        let sd = Poisson::sd(lambda);

        assert_approx_eq!(2.8284448023604774, sd);
    }

    #[test]
    fn poisson_sd_test2() {
        let lambda = 10.98_f64;

        let sd = Poisson::sd(lambda);

        assert_approx_eq!(3.3136083051561784, sd);
    }

    #[test]
    fn poisson_sd_test3() {
        let lambda = 6.1_f64;

        let sd = Poisson::sd(lambda);

        assert_approx_eq!(2.469817807045693, sd);
    }

    #[test]
    fn poisson_sd_test4() {
        let lambda = 1_f64;

        let sd = Poisson::sd(lambda);

        assert_approx_eq!(1.0, sd);
    }
}
