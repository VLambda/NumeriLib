use numerilib::stats::distr::GeometricTrials;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn geometric_trials_median_test1() {
        let probability = 0.5;

        let geometric_trials = GeometricTrials::median(probability);

        assert_approx_eq!(1_f64, geometric_trials);
    }

    #[test]
    pub fn geometric_trials_median_test2() {
        let probability = 0.975;

        let geometric_trials = GeometricTrials::median(probability);

        assert_approx_eq!(1_f64, geometric_trials);
    }

    #[test]
    pub fn geometric_trials_median_test3() {
        let probability = 0.02;

        let geometric_trials = GeometricTrials::median(probability);

        assert_approx_eq!(35_f64, geometric_trials);
    }

    #[test]
    pub fn geometric_trials_median_test4() {
        let probability = 0.255;

        let geometric_trials = GeometricTrials::median(probability);

        assert_approx_eq!(3_f64, geometric_trials);
    }
}
