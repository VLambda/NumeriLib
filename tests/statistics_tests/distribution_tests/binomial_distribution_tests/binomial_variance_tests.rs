use numerilib::stats::distr::Binomial;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn binomial_variance_test1() {
        let trials = 8.0;
        let probability = 0.125;

        let variance = Binomial::variance(trials, probability);

        assert_approx_eq!(0.875_f64, variance);
    }

    #[test]
    fn binomial_variance_test2() {
        let trials = 70_f64;
        let probability = 1_f64 / 365_f64;

        let variance = Binomial::variance(trials, probability);

        assert_approx_eq!(0.19125539500844435_f64, variance);
    }

    #[test]
    fn binomial_variance_test3() {
        let trials = 100_f64;
        let probability = 1_f64 / 3_f64;

        let variance = Binomial::variance(trials, probability);

        assert_approx_eq!(22.22222222222222, variance);
    }

    #[test]
    fn binomial_variance_test4() {
        let trials = 100_f64;
        let probability = 0.5;

        let variance = Binomial::variance(trials, probability);

        assert_approx_eq!(25_f64, variance);
    }
}
