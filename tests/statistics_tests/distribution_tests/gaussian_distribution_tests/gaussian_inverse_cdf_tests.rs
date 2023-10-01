use numerilib::stats::distr::Gaussian;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn gaussian_tailcdf_test1() {
        let area = 0.975;
        let mean = 0.0;
        let sd = 1.0;
        let tail = "Left";

        let normalcdf = Gaussian::inv(area, mean, sd, tail).unwrap();

        assert_approx_eq!(1.9599639845400534, normalcdf);
    }

    #[test]
    pub fn gaussian_tailcdf_test2() {
        let area = 0.975;
        let mean = 0.0;
        let sd = 1.0;
        let tail = "Right";

        let normalcdf = Gaussian::inv(area, mean, sd, tail).unwrap();

        assert_approx_eq!(-1.9599639845400534, normalcdf);
    }

    #[test]
    pub fn gaussian_tailcdf_test3() {
        let area = 0.75;
        let mean = 8.0;
        let sd = 1.0;
        let tail = "Left";

        let normalcdf = Gaussian::inv(area, mean, sd, tail).unwrap();

        assert_approx_eq!(8.674489750196082, normalcdf);
    }

    #[test]
    pub fn gaussian_tailcdf_test4() {
        let area = 0.56;
        let mean = 8.0;
        let sd = 1.0;
        let tail = "Right";

        let normalcdf = Gaussian::inv(area, mean, sd, tail).unwrap();

        assert_approx_eq!(7.849030784503223, normalcdf);
    }
}
