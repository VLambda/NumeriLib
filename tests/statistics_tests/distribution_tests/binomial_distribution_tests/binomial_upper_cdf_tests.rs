use numerilib::stats::distr::Binomial;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn binomial_cdf_test1() {
        let trials = 8.0;
        let probability = 0.125;
        let x = 2.0;

        let cdf = Binomial::ucdf(trials, probability, x);

        assert_approx_eq!(0.2636951804161072, cdf);
    }

    #[test]
    fn binomial_cdf_test2() {
        let trials = 70_f64;
        let probability = 1_f64 / 365_f64;
        let x = 2_f64;

        let cdf = Binomial::ucdf(trials, probability, x);

        assert_approx_eq!(0.01602358404143703, cdf);
    }

    #[test]
    fn binomial_cdf_test3() {
        let trials = 100_f64;
        let probability = 0.5;
        let x = 40_f64;

        let cdf = Binomial::ucdf(trials, probability, x);

        assert_approx_eq!(0.9823998998911473, cdf);
    }

    #[test]
    fn binomial_cdf_test4() {
        let trials = 100_f64;
        let probability = 0.5;
        let x = 50_f64;

        let cdf = Binomial::ucdf(trials, probability, x);

        assert_approx_eq!(0.5397946186935897, cdf);
    }
}
