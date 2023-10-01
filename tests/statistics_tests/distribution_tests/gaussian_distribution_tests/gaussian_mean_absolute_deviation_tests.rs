use numerilib::stats::distr::Gaussian;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn gaussian_tailcdf_test1() {
        let sigma = 1.0;

        let normalcdf = Gaussian::mad(sigma);

        assert_approx_eq!(0.6744897501960819, normalcdf);
    }

    #[test]
    pub fn gaussian_tailcdf_test2() {
        let sigma = 8.0;

        let normalcdf = Gaussian::mad(sigma);

        assert_approx_eq!(5.395918001568655, normalcdf);
    }

    #[test]
    pub fn gaussian_tailcdf_test3() {
        let sigma = 0.0;

        let normalcdf = Gaussian::mad(sigma);

        assert_approx_eq!(0.0, normalcdf);
    }

    #[test]
    pub fn gaussian_tailcdf_test4() {
        let sigma = -1.0;

        let normalcdf = Gaussian::mad(sigma);

        assert_approx_eq!(-0.6744897501960819, normalcdf);
    }
}
