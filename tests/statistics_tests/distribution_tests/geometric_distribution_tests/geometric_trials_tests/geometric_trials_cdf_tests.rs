use numerilib::stats::distr::GeometricTrials;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn geometric_trials_cdf_test1() {
        let probability = 0.5;
        let trials = 5_f64;

        let geometric_trials = GeometricTrials::cdf(probability, trials);

        assert_approx_eq!(0.96875, geometric_trials);
    }

    #[test]
    pub fn geometric_trials_cdf_test2() {
        let probability = 0.975;
        let trials = 1000_f64;

        let geometric_trials = GeometricTrials::cdf(probability, trials);

        assert_approx_eq!(1_f64, geometric_trials);
    }

    #[test]
    pub fn geometric_trials_cdf_test3() {
        let probability = 0.02;
        let trials = 10_f64;

        let geometric_trials = GeometricTrials::cdf(probability, trials);

        assert_approx_eq!(0.1829271931124533, geometric_trials);
    }

    #[test]
    pub fn geometric_trials_cdf_test4() {
        let probability = 0.255;
        let trials = 15_f64;

        let geometric_trials = GeometricTrials::cdf(probability, trials);

        assert_approx_eq!(0.9879122883561265, geometric_trials);
    }
}
