use numerilib::stats::distr::Cauchy;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn cauchy_cdf_test1() {
        let location = 0_f64;
        let scale = 1_f64;
        let x = 2.5;

        let cdf = Cauchy::cdf(location, scale, x);

        assert_approx_eq!(0.8788810584091566, cdf);
    }

    #[test]
    pub fn cauchy_cdf_test2() {
        let location = 2_f64;
        let scale = 4_f64;
        let x = 2.5;

        let cdf = Cauchy::cdf(location, scale, x);

        assert_approx_eq!(0.5395834241605656, cdf);
    }

    #[test]
    pub fn cauchy_cdf_test3() {
        let location = 3_f64;
        let scale = 1_f64;
        let x = 2.5;

        let cdf = Cauchy::cdf(location, scale, x);

        assert_approx_eq!(0.35241638234956674, cdf);
    }

    #[test]
    pub fn cauchy_cdf_test4() {
        let location = 9_f64;
        let scale = 5_f64;
        let x = 8_f64;

        let cdf = Cauchy::cdf(location, scale, x);

        assert_approx_eq!(0.4371670418109988, cdf);
    }
}
