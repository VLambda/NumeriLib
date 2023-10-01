use numerilib::stats::distr::Gaussian;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn gaussian_tailcdf_test1() {
        let lower = 0.0;
        let upper = 1.0;
        let mean = 0.0;
        let sd = 1.0;

        let normalcdf = Gaussian::tailcdf(lower, upper, mean, sd);

        assert_approx_eq!(0.34134474606854287, normalcdf);
    }

    #[test]
    pub fn gaussian_tailcdf_test2() {
        let lower = -1.96;
        let upper = 1.96;
        let mean = 0.0;
        let sd = 1.0;

        let normalcdf = Gaussian::tailcdf(lower, upper, mean, sd);

        assert_approx_eq!(0.950004052436277, normalcdf);
    }

    #[test]
    pub fn gaussian_tailcdf_test3() {
        let lower = 0_f64;
        let upper = -2.1482;
        let mean = 8.0;
        let sd = 1.0;

        let normalcdf = Gaussian::tailcdf(lower, upper, mean, sd);

        assert_approx_eq!(0_f64, normalcdf);
    }

    #[test]
    pub fn gaussian_tailcdf_test4() {
        let lower = -0.5;
        let upper = 0.5;
        let mean = 0.0;
        let sd = 8.0;

        let normalcdf = Gaussian::tailcdf(lower, upper, mean, sd);

        assert_approx_eq!(0.04983533805849444, normalcdf);
    }
}
