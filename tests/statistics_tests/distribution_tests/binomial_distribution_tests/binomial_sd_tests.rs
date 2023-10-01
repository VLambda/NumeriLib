use numerilib::stats::distr::Binomial;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn binomial_sd_test1() {
        let trials = 8.0;
        let probability = 0.125;

        let mode = Binomial::sd(trials, probability);

        assert_approx_eq!(0.9354143466934853, mode);
    }

    #[test]
    fn binomial_sd_test2() {
        let trials = 70_f64;
        let probability = 1_f64 / 365_f64;

        let mode = Binomial::sd(trials, probability);

        assert_approx_eq!(0.4373275603119981, mode);
    }

    #[test]
    fn binomial_sd_test3() {
        let trials = 100_f64;
        let probability = 0.5;

        let mode = Binomial::sd(trials, probability);

        assert_approx_eq!(5.0, mode);
    }

    #[test]
    fn binomial_sd_test4() {
        let trials = 100_f64;
        let probability = 0.5;

        let mode = Binomial::sd(trials, probability);

        assert_approx_eq!(5.0, mode);
    }
}
