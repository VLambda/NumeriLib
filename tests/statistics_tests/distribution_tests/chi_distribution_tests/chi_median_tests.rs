use numerilib::stats::distr::Chi;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn chi_median_test1() {
        let df = 5.0;

        let median = Chi::median(df);

        assert_approx_eq!(2.0886656040369336, median);
    }

    #[test]
    fn chi_median_test2() {
        let df = 7.0;

        let median = Chi::median(df);

        assert_approx_eq!(2.5207684139169624, median);
    }

    #[test]
    fn chi_median_test3() {
        let df = 9.0;

        let median = Chi::median(df);

        assert_approx_eq!(2.889577608921529, median);
    }

    #[test]
    fn chi_median_test4() {
        let df = 11.0;

        let median = Chi::median(df);

        assert_approx_eq!(3.2166303258391316, median);
    }
}
