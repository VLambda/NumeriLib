use numerilib::stats::distr::GeometricTrials;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn geometric_trials_kurtosis_test1() {
        let probability = 0.5;

        let geometric_trials = GeometricTrials::kurtosis(probability);

        assert_approx_eq!(6.5, geometric_trials);
    }

    #[test]
    pub fn geometric_trials_kurtosis_test2() {
        let probability = 0.975;

        let geometric_trials = GeometricTrials::kurtosis(probability);

        assert_approx_eq!(44.02499999999996, geometric_trials);
    }

    #[test]
    pub fn geometric_trials_kurtosis_test3() {
        let probability = 0.02;

        let geometric_trials = GeometricTrials::kurtosis(probability);

        assert_approx_eq!(6.000408163265306, geometric_trials);
    }

    #[test]
    pub fn geometric_trials_kurtosis_test4() {
        let probability = 0.255;

        let geometric_trials = GeometricTrials::kurtosis(probability);

        assert_approx_eq!(6.087281879194631, geometric_trials);
    }
}
