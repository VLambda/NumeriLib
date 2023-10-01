use numerilib::stats::distr::Binomial;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn binomial_kurtosis_test1() {
        let trials = 8.0;
        let probability = 0.125;

        let kurtosis = Binomial::kurtosis(trials, probability);

        assert_approx_eq!(0.39285714285714285, kurtosis);
    }

    #[test]
    fn binomial_kurtosis_test2() {
        let trials = 70_f64;
        let probability = 1_f64 / 365_f64;

        let kurtosis = Binomial::kurtosis(trials, probability);

        assert_approx_eq!(5.142896389324961, kurtosis);
    }

    #[test]
    fn binomial_kurtosis_test3() {
        let trials = 100_f64;
        let probability = 0.5;

        let kurtosis = Binomial::kurtosis(trials, probability);

        assert_approx_eq!(-0.02, kurtosis);
    }

    #[test]
    fn binomial_kurtosis_test4() {
        let trials = 100_f64;
        let probability = 0.5;

        let kurtosis = Binomial::kurtosis(trials, probability);

        assert_approx_eq!(-0.02, kurtosis);
    }
}
