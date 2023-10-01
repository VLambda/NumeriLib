use numerilib::stats::distr::Chi2;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn chi_sqaured_kurtosis_test1() {
        let df = 4.0;

        let kurtosis = Chi2::kurtosis(df);

        assert_approx_eq!(12_f64 / 4_f64, kurtosis);
    }

    #[test]
    pub fn chi_sqaured_kurtosis_test2() {
        let df = 7.0;

        let kurtosis = Chi2::kurtosis(df);

        assert_approx_eq!(12_f64 / 7_f64, kurtosis);
    }

    #[test]
    pub fn chi_sqaured_kurtosis_test3() {
        let df = 10.0;

        let kurtosis = Chi2::kurtosis(df);

        assert_approx_eq!(12_f64 / 10_f64, kurtosis);
    }

    #[test]
    pub fn chi_sqaured_kurtosis_test4() {
        let df = 13.0;

        let kurtosis = Chi2::kurtosis(df);

        assert_approx_eq!(12_f64 / 13_f64, kurtosis);
    }
}
