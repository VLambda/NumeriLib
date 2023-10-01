use numerilib::stats::distr::Chi;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn chi_kurtosis_test1() {
        let df = 5.0;

        let kurtosis = Chi::kurtosis(df);

        assert_approx_eq!(0.03698105871274147, kurtosis);
    }

    #[test]
    fn chi_kurtosis_test2() {
        let df = 7.0;

        let kurtosis = Chi::kurtosis(df);

        assert_approx_eq!(0.018104405989318652, kurtosis);
    }

    #[test]
    fn chi_kurtosis_test3() {
        let df = 9.0;

        let kurtosis = Chi::kurtosis(df);

        assert_approx_eq!(0.01063817256382387, kurtosis);
    }

    #[test]
    fn chi_kurtosis_test4() {
        let df = 11.0;

        let kurtosis = Chi::kurtosis(df);

        assert_approx_eq!(0.006973865929382074, kurtosis);
    }
}
