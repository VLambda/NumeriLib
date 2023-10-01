use numerilib::stats::distr::GeometricFailure;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn geometric_failures_median_test1() {
        let probability = 0.5;

        let geometric_failures = GeometricFailure::median(probability);

        assert_approx_eq!(0_f64, geometric_failures);
    }

    #[test]
    pub fn geometric_failures_median_test2() {
        let probability = 0.975;

        let geometric_failures = GeometricFailure::median(probability);

        assert_approx_eq!(0_f64, geometric_failures);
    }

    #[test]
    pub fn geometric_failures_median_test3() {
        let probability = 0.02;

        let geometric_failures = GeometricFailure::median(probability);

        assert_approx_eq!(34_f64, geometric_failures);
    }

    #[test]
    pub fn geometric_failures_median_test4() {
        let probability = 0.255;

        let geometric_failures = GeometricFailure::median(probability);

        assert_approx_eq!(2_f64, geometric_failures);
    }
}
