use numerilib::stats::distr::GeometricTrials;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn geometric_trials_sd_test1() {
        let probability = 0.5;

        let geometric_trials = GeometricTrials::sd(probability);

        assert_approx_eq!(std::f64::consts::SQRT_2, geometric_trials);
    }

    #[test]
    pub fn geometric_trials_sd_test2() {
        let probability = 0.975;

        let geometric_trials = GeometricTrials::sd(probability);

        assert_approx_eq!(0.16216808513684006, geometric_trials);
    }

    #[test]
    pub fn geometric_trials_sd_test3() {
        let probability = 0.02;

        let geometric_trials = GeometricTrials::sd(probability);

        assert_approx_eq!(49.49747468305833, geometric_trials);
    }

    #[test]
    pub fn geometric_trials_sd_test4() {
        let probability = 0.255;
        let trials = 15_f64;

        let geometric_trials = GeometricTrials::sd(probability);

        assert_approx_eq!(3.3848385297317782, geometric_trials);
    }
}
