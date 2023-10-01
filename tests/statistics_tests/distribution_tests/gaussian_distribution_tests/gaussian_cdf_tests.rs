use numerilib::stats::distr::Gaussian;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn gaussian_cdf_test1() {
        let x_value = 1_f64;
        let mean = 0.0;
        let sd = 1.0;

        let normalcdf = Gaussian::cdf(x_value, mean, sd);

        assert_approx_eq!(0.15865525393145713, normalcdf);
    }

    #[test]
    pub fn gaussian_cdf_test2() {
        let x_value = 0.5;
        let mean = 0.0;
        let sd = 1.0;

        let normalcdf = Gaussian::cdf(x_value, mean, sd);

        assert_approx_eq!(0.3085375387259869, normalcdf);
    }

    #[test]
    pub fn gaussian_cdf_test3() {
        let x_value = 2.1482;
        let mean = 8.0;
        let sd = 1.0;

        let normalcdf = Gaussian::cdf(x_value, mean, sd);

        assert_approx_eq!(0.9999999999999999, normalcdf);
    }

    #[test]
    pub fn gaussian_cdf_test4() {
        let x_value = 0.5;
        let mean = 0.0;
        let sd = 8.0;

        let normalcdf = Gaussian::cdf(x_value, mean, sd);

        assert_approx_eq!(0.47508233097075275, normalcdf);
    }
}
