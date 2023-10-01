use numerilib::stats::distr::Poisson;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn poisson_kurtosis_test1() {
        let lambda = 8.0001_f64;

        let kurtosis = Poisson::kurtosis(lambda);

        assert_approx_eq!(0.124998437519531, kurtosis);
    }

    #[test]
    fn poisson_kurtosis_test2() {
        let lambda = 10.98_f64;

        let kurtosis = Poisson::kurtosis(lambda);

        assert_approx_eq!(0.09107468123861566, kurtosis);
    }

    #[test]
    fn poisson_kurtosis_test3() {
        let lambda = 6.1_f64;

        let kurtosis = Poisson::kurtosis(lambda);

        assert_approx_eq!(0.1639344262295082, kurtosis);
    }

    #[test]
    fn poisson_kurtosis_test4() {
        let lambda = 1_f64;

        let kurtosis = Poisson::kurtosis(lambda);

        assert_approx_eq!(1.0, kurtosis);
    }
}
