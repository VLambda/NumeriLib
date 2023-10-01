use numerilib::stats::distr::GeometricTrials;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn geometric_trials_mean_test1() {
        let probability = 0.5;

        let geometric_trials = GeometricTrials::mean(probability);

        assert_approx_eq!(2_f64, geometric_trials);
    }

    #[test]
    pub fn geometric_trials_mean_test2() {
        let probability = 0.975;

        let geometric_trials = GeometricTrials::mean(probability);

        assert_approx_eq!(1.0256410256410258, geometric_trials);
    }

    #[test]
    pub fn geometric_trials_mean_test3() {
        let probability = 0.02;

        let geometric_trials = GeometricTrials::mean(probability);

        assert_approx_eq!(50_f64, geometric_trials);
    }

    #[test]
    pub fn geometric_trials_mean_test4() {
        let probability = 0.255;

        let geometric_trials = GeometricTrials::mean(probability);

        assert_approx_eq!(3.9215686274509802, geometric_trials);
    }
}
