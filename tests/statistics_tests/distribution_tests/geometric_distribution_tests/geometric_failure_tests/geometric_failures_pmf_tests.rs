use numerilib::stats::distr::GeometricFailure;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn geometric_failures_pmf_test1() {
        let probability = 0.5;
        let trials = 5_f64;

        let geometric_failures = GeometricFailure::pmf(probability, trials);

        assert_approx_eq!(0.015625, geometric_failures);
    }

    #[test]
    pub fn geometric_failures_pmf_test2() {
        let probability = 0.975;
        let trials = 1000_f64;

        let geometric_failures = GeometricFailure::pmf(probability, trials);

        assert_approx_eq!(0_f64, geometric_failures);
    }

    #[test]
    pub fn geometric_failures_pmf_test3() {
        let probability = 0.02;
        let trials = 10_f64;

        let geometric_failures = GeometricFailure::pmf(probability, trials);

        assert_approx_eq!(0.016341456137750933, geometric_failures);
    }

    #[test]
    pub fn geometric_failures_pmf_test4() {
        let probability = 0.255;
        let trials = 15_f64;

        let geometric_failures = GeometricFailure::pmf(probability, trials);

        assert_approx_eq!(0.003082366469187727, geometric_failures);
    }
}
