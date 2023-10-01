use numerilib::stats::distr::Fisher;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn fisher_cdf_test1() {
        let d1 = 4_f64;
        let d2 = 8_f64;
        let lower = 0_f64;
        let upper = 5_f64;

        let cdf = Fisher::tailcdf(lower, upper, d1, d2);

        assert_approx_eq!(0.974296424119155, cdf);
    }

    #[test]
    pub fn fisher_cdf_test2() {
        let d1 = 8_f64;
        let d2 = 4_f64;
        let lower = -5_f64;
        let upper = 5_f64;

        let cdf = Fisher::tailcdf(lower, upper, d1, d2);

        assert_approx_eq!(0.9309068317222611, cdf);
    }

    #[test]
    pub fn fisher_cdf_test3() {
        let d1 = 7_f64;
        let d2 = 10_f64;
        let lower = 2_f64;
        let upper = -3_f64;

        let cdf = Fisher::tailcdf(lower, upper, d1, d2);

        assert_approx_eq!(-0.8454966997439163, cdf);
    }

    #[test]
    pub fn fisher_cdf_test4() {
        let d1 = 16_f64;
        let d2 = 2_f64;
        let lower = 7_f64;
        let upper = 7.5;

        let cdf = Fisher::tailcdf(lower, upper, d1, d2);

        assert_approx_eq!(-0.01159830972767184, cdf);
    }
}
