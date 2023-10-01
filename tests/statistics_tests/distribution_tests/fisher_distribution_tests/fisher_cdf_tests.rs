use numerilib::stats::distr::Fisher;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn fisher_cdf_test1() {
        let d1 = 4.0;
        let d2 = 8.0;
        let x = 0.5;

        let cdf = Fisher::cdf(x, d1, d2);

        assert_approx_eq!(0.26272000000249174, cdf);
    }

    #[test]
    pub fn fisher_cdf_test2() {
        let d1 = 8.0;
        let d2 = 4.0;
        let x = 0.5;

        let cdf = Fisher::cdf(x, d1, d2);

        assert_approx_eq!(0.1875, cdf);
    }

    #[test]
    pub fn fisher_cdf_test3() {
        let d1 = 7.0;
        let d2 = 10.0;
        let x = 0.5;

        let cdf = Fisher::cdf(x, d1, d2);

        assert_approx_eq!(0.1849568534092207, cdf);
    }

    #[test]
    pub fn fisher_cdf_test4() {
        let d1 = 16.0;
        let d2 = 2.0;
        let x = 0.5;

        let cdf = Fisher::cdf(x, d1, d2);

        assert_approx_eq!(0.1677721599735872, cdf);
    }
}
