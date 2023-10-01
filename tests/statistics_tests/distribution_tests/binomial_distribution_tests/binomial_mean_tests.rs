use numerilib::stats::distr::Binomial;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn binomial_mean_test1() {
        let trials = 8.0;
        let probability = 0.125;

        let mean = Binomial::mean(probability, trials);

        assert_approx_eq!(1_f64, mean);
    }

    #[test]
    fn binomial_mean_test2() {
        let trials = 70_f64;
        let probability = 1_f64 / 365_f64;

        let mean = Binomial::mean(probability, trials);

        assert_approx_eq!(0.1917808219178082_f64, mean);
    }

    #[test]
    fn binomial_mean_test3() {
        let trials = 100_f64;
        let probability = 0.5;

        let mean = Binomial::mean(probability, trials);

        assert_approx_eq!(50_f64, mean);
    }

    #[test]
    fn binomial_mean_test4() {
        let trials = 100_f64;
        let probability = 0.5;

        let mean = Binomial::mean(probability, trials);

        assert_approx_eq!(50_f64, mean);
    }
}
