use numerilib::stats::distr::Chi2;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    #[test]
    pub fn chi_sqaured_cdf_test1() {
        let df = 4.0;

        let sd = Chi2::sd(df);

        assert_approx_eq!(2_f64 * std::f64::consts::SQRT_2, sd);
    }

    #[test]
    pub fn chi_sqaured_cdf_test2() {
        let df = 7.0;

        let sd = Chi2::sd(df);

        assert_approx_eq!(7_f64.sqrt() * std::f64::consts::SQRT_2, sd);
    }

    #[test]
    pub fn chi_sqaured_cdf_test3() {
        let df = 10.0;

        let sd = Chi2::sd(df);

        assert_approx_eq!(std::f64::consts::SQRT_2 * 10_f64.sqrt(), sd);
    }

    #[test]
    pub fn chi_sqaured_cdf_test4() {
        let df = 13.0;

        let sd = Chi2::sd(df);

        assert_approx_eq!(std::f64::consts::SQRT_2 * 13_f64.sqrt(), sd);
    }
}
