use numerilib::stats::distr::Binomial;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn binomial_cdf_test1() {
        let trials = 8.0;
        let probability = 0.125;
        let area = 0.9326527714729309;

        let invcdf = Binomial::inv(area, trials, probability);

        assert_approx_eq!(2_f64, invcdf);
    }

    #[test]
    fn binomial_cdf_test2() {
        let trials = 70_f64;
        let probability = 1_f64 / 365_f64;
        let area = 0.9990185935332583;

        let invcdf = Binomial::inv(area, trials, probability);

        assert_approx_eq!(2_f64, invcdf);
    }

    #[test]
    fn binomial_cdf_test3() {
        let trials = 100_f64;
        let probability = 0.5;
        let area = 0.028443966820490378;

        let invcdf = Binomial::inv(area, trials, probability);

        assert_approx_eq!(40_f64, invcdf);
    }

    #[test]
    fn binomial_cdf_test4() {
        let trials = 100_f64;
        let probability = 0.5;
        let area = 0.5397946186935897;

        let cdf = Binomial::inv(area, trials, probability);

        assert_approx_eq!(50_f64, cdf);
    }
}
