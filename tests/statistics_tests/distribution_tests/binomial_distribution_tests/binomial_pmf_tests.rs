use numerilib::stats::distr::Binomial;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn binomial_pmf_test1() {
        let trials = 8.0;
        let probability = 0.125;
        let x = 2.0;

        let pdf = Binomial::pmf(trials, probability, x);

        assert_approx_eq!(0.19634795188903809, pdf);
    }

    #[test]
    pub fn binomial_pmf_test2() {
        let trials = 10_f64;
        let probability = 0.25;
        let x = 4_f64;

        let pdf = Binomial::pmf(trials, probability, x);

        assert_approx_eq!(0.1459980011, pdf);
    }

    #[test]
    pub fn binomial_pmf_test3() {
        let trials = 70_f64;
        let probability = 1_f64 / 365_f64;
        let x = 2_f64;

        let pdf = Binomial::pmf(trials, probability, x);

        assert_approx_eq!(0.0150421776, pdf);
    }

    #[test]
    pub fn binomial_pmf_test4() {
        let trials = 100_f64;
        let probability = 0.5;
        let x = 60_f64;

        let pdf = Binomial::pmf(trials, probability, x);

        assert_approx_eq!(0.0108438667, pdf);
    }
}
