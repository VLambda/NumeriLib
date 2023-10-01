use numerilib::stats::distr::GeometricTrials;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn geometric_trials_variance_test1() {
        let probability = 0.5;

        let geometric_trials = GeometricTrials::variance(probability);

        assert_approx_eq!(2_f64, geometric_trials);
    }

    #[test]
    pub fn geometric_trials_variance_test2() {
        let probability = 0.975;

        let geometric_trials = GeometricTrials::variance(probability);

        assert_approx_eq!(0.0262984878369494, geometric_trials);
    }

    #[test]
    pub fn geometric_trials_variance_test3() {
        let probability = 0.02;

        let geometric_trials = GeometricTrials::variance(probability);

        assert_approx_eq!(2450_f64, geometric_trials);
    }

    #[test]
    pub fn geometric_trials_variance_test4() {
        let probability = 0.255;

        let geometric_trials = GeometricTrials::variance(probability);

        assert_approx_eq!(11.457131872356786, geometric_trials);
    }
}
