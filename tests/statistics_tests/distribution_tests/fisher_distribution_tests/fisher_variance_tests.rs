use numerilib::stats::distr::Fisher;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn fisher_variance_test1() {
        let d1 = 4.0;
        let d2 = 8.0;

        let variance = Fisher::variance(d1, d2);

        assert_approx_eq!(20_f64 / 9_f64, variance);
    }

    #[test]
    pub fn fisher_variance_test2() {
        let d1 = 8.0;
        let d2 = 6.0;

        let variance = Fisher::variance(d1, d2);

        assert_approx_eq!(3.375, variance);
    }

    #[test]
    pub fn fisher_variance_test3() {
        let d1 = 7.0;
        let d2 = 10.0;

        let variance = Fisher::variance(d1, d2);

        assert_approx_eq!(1.1160714285714286, variance);
    }

    #[test]
    pub fn fisher_variance_test4() {
        let d1 = 16.0;
        let d2 = 9.0;

        let variance = Fisher::variance(d1, d2);

        assert_approx_eq!(0.9505102040816327, variance);
    }
}
