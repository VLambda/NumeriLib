use numerilib::stats::distr::Binomial;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn binomial_skewness_test1() {
        let trials = 8.0;
        let probability = 0.125;

        let skewness = Binomial::skewness(trials, probability);

        assert_approx_eq!(0.8017837257372732, skewness);
    }

    #[test]
    fn binomial_skewness_test2() {
        let trials = 70_f64;
        let probability = 1_f64 / 365_f64;

        let skewness = Binomial::skewness(trials, probability);

        assert_approx_eq!(2.2740861500603686, skewness);
    }

    #[test]
    fn binomial_skewness_test3() {
        let trials = 100_f64;
        let probability = 0.5;

        let skewness = Binomial::skewness(trials, probability);

        assert_approx_eq!(0.0, skewness);
    }

    #[test]
    fn binomial_skewness_test4() {
        let trials = 100_f64;
        let probability = 0.5;

        let skewness = Binomial::skewness(trials, probability);

        assert_approx_eq!(0.0, skewness);
    }
}
