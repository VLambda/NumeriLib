use numerilib::stats::distr::GeometricTrials;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn geometric_trials_skewness_test1() {
        let probability = 0.5;

        let geometric_trials = GeometricTrials::skewness(probability);

        assert_approx_eq!(2.1213203435596424, geometric_trials);
    }

    #[test]
    pub fn geometric_trials_skewness_test2() {
        let probability = 0.975;

        let geometric_trials = GeometricTrials::skewness(probability);

        assert_approx_eq!(6.482669203345175, geometric_trials);
    }

    #[test]
    pub fn geometric_trials_skewness_test3() {
        let probability = 0.02;

        let geometric_trials = GeometricTrials::skewness(probability);

        assert_approx_eq!(2.000102038213377, geometric_trials);
    }

    #[test]
    pub fn geometric_trials_skewness_test4() {
        let probability = 0.255;

        let geometric_trials = GeometricTrials::skewness(probability);

        assert_approx_eq!(2.021702717808588, geometric_trials);
    }
}
