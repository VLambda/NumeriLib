use numerilib::stats::distr::Chi;

#[cfg(test)]
pub mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn chi_variance_test1() {
        let df = 5.0;

        let variance = Chi::variance(df);

        assert_approx_eq!(0.4729260631780736, variance);
    }

    #[test]
    fn chi_variance_test2() {
        let df = 7.0;

        let variance = Chi::variance(df);

        assert_approx_eq!(0.48101353096832433, variance);
    }

    #[test]
    fn chi_variance_test3() {
        let df = 9.0;

        let variance = Chi::variance(df);

        assert_approx_eq!(0.4854054281962181, variance);
    }

    #[test]
    fn chi_variance_test4() {
        let df = 11.0;

        let variance = Chi::variance(df);

        assert_approx_eq!(0.48815484961912503, variance);
    }
}
