use numerilib::stats::distr::Chi2;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn chi_sqaured_skewness_test1() {
        let df = 4.0;

        let skewness = Chi2::skewness(df);

        assert_approx_eq!(std::f64::consts::SQRT_2, skewness);
    }

    #[test]
    pub fn chi_sqaured_skewness_test2() {
        let df = 7.0;

        let skewness = Chi2::skewness(df);

        assert_approx_eq!(1.0690449676496976, skewness);
    }

    #[test]
    pub fn chi_sqaured_skewness_test3() {
        let df = 10.0;

        let skewness = Chi2::skewness(df);

        assert_approx_eq!(0.8944271909999159, skewness);
    }

    #[test]
    pub fn chi_sqaured_skewness_test4() {
        let df = 13.0;

        let skewness = Chi2::skewness(df);

        assert_approx_eq!(0.7844645405527362, skewness);
    }
}
