use numerilib::stats::distr::GeometricTrials;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn geometric_trials_pmf_test1() {
        let probability = 0.5;
        let trials = 5_f64;

        let geometric_trials = GeometricTrials::pmf(probability, trials);

        assert_approx_eq!(0.03125, geometric_trials);
    }

    #[test]
    pub fn geometric_trials_pmf_test2() {
        let probability = 0.975;
        let trials = 1000_f64;

        let geometric_trials = GeometricTrials::pmf(probability, trials);

        assert_approx_eq!(0_f64, geometric_trials);
    }

    #[test]
    pub fn geometric_trials_pmf_test3() {
        let probability = 0.02;
        let trials = 10_f64;

        let geometric_trials = GeometricTrials::pmf(probability, trials);

        assert_approx_eq!(0.016674955242602995, geometric_trials);
    }

    #[test]
    pub fn geometric_trials_pmf_test4() {
        let probability = 0.255;
        let trials = 15_f64;

        let geometric_trials = GeometricTrials::pmf(probability, trials);

        assert_approx_eq!(0.0041374046566278215, geometric_trials);
    }
}
