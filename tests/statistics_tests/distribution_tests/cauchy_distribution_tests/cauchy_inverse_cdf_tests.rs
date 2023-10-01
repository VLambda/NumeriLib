use numerilib::stats::distr::Cauchy;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn cauchy_inverse_cdf_test1() {
        let location = 0_f64;
        let scale = 1_f64;
        let p = 0.8788810584091566;

        let x = Cauchy::inv(location, scale, p);

        assert_approx_eq!(2.5, x);
    }

    #[test]
    pub fn cauchy_inverse_cdf_test2() {
        let location = 2_f64;
        let scale = 4_f64;
        let p = 0.5395834241605656;

        let x = Cauchy::inv(location, scale, p);

        assert_approx_eq!(2.5, x);
    }

    #[test]
    pub fn cauchy_inverse_cdf_test3() {
        let location = 3_f64;
        let scale = 1_f64;
        let p = 0.35241638234956674;

        let x = Cauchy::inv(location, scale, p);

        assert_approx_eq!(2.5, x);
    }

    #[test]
    pub fn cauchy_inverse_cdf_test4() {
        let location = 9_f64;
        let scale = 5_f64;
        let p = 0.4371670418109988;

        let x = Cauchy::inv(location, scale, p);

        assert_approx_eq!(8_f64, x);
    }
}
