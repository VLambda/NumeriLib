use numerilib::stats::distr::GeometricFailure;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn geometric_failures_mean_test1() {
        let probability = 0.5;

        let geometric_failures = GeometricFailure::mean(probability);

        assert_approx_eq!(1_f64, geometric_failures);
    }

    #[test]
    pub fn geometric_failures_mean_test2() {
        let probability = 0.975;

        let geometric_failures = GeometricFailure::mean(probability);

        assert_approx_eq!(0.025641025641025664, geometric_failures);
    }

    #[test]
    pub fn geometric_failures_mean_test3() {
        let probability = 0.02;

        let geometric_failures = GeometricFailure::mean(probability);

        assert_approx_eq!(49_f64, geometric_failures);
    }

    #[test]
    pub fn geometric_failures_mean_test4() {
        let probability = 0.255;

        let geometric_failures = GeometricFailure::mean(probability);

        assert_approx_eq!(2.9215686274509802, geometric_failures);
    }
}
